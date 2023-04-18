<script setup lang="ts">
import { useRepoStore, useModalsStore } from "@stores";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { SearchBar } from "@components/Modal";

const activeBranchName = ref<null | string>(null);
const repoName = ref<null | string>(null);
const localBranchesNames = ref<null | string[]>(null);
const remoteBranchesNames = ref<null | string[]>(null);
const repoStore = useRepoStore();
const modalsStore = useModalsStore();

const pendingCommitsToPush = ref<null | number>(null);
const pendingCommitsToPull = ref<null | number>(null);

const isFetching = ref(false);
const isPulling = ref(false);
const isPushing = ref(false);

async function openRepo(path: string | null) {
  if (!path) {
    return;
  }
  await invoke("open", { path });
  repoName.value = await invoke("get_repo_name");

  await invoke("db_insert", {
    key: repoName.value,
    value: path,
  });
  repoStore.setRepo(path);
  await resfreshBranches();
  await invoke("write_last_opened_repo", {
    key: path,
  });
  await getPendingCommitsToPush();
  await getPendingCommitsToPull();
}

async function handleOpenFile() {
  const file = await open({
    directory: true,
    multiple: false,
  });
  if (file && !Array.isArray(file)) {
    openRepo(file);
  }
}

watch(repoStore, async () => {
  await openRepo(repoStore.repo);
  await getPendingCommitsToPush();
  await getPendingCommitsToPull();
});

async function resfreshBranches() {
  localBranchesNames.value = await invoke("find_branches", { filter: "Local" });
  remoteBranchesNames.value = await invoke("find_branches", {
    filter: "Remote",
  });
  activeBranchName.value = await invoke<string>("get_current_branch_name");
  repoStore.setActiveBranch(activeBranchName.value);
}

async function getPendingCommitsToPush() {
  try {
    const _pendingCommitsToPush = await invoke<number>(
      "get_pending_commits_to_push"
    );
    pendingCommitsToPush.value = _pendingCommitsToPush;
  } catch (e) {
    pendingCommitsToPush.value = null;
  }
}

async function getPendingCommitsToPull() {
  try {
    const _pendingCommitsToPull = await invoke<number>(
      "get_pending_commits_to_pull"
    );
    pendingCommitsToPull.value = _pendingCommitsToPull;
  } catch (e) {
    pendingCommitsToPull.value = null;
  }
}

async function checkoutBranch(branch: string) {
  await invoke("checkout_branch", { branchName: branch });
  activeBranchName.value = branch;
  repoStore.setActiveBranch(activeBranchName.value);
}
async function getRemotes() {
  console.log("remotes:", await invoke("get_remotes"));
}
async function fetchRemote() {
  try {
    isFetching.value = true;
    await invoke("fetch_remote");
    await resfreshBranches();
  } catch (e) {
    console.log(e);
  } finally {
    isFetching.value = false;
  }
}
async function pushRemote() {
  try {
    isPushing.value = true;
    await invoke("push_remote");
    await resfreshBranches();
  } catch (e) {
    console.log(e);
  } finally {
    isPushing.value = false;
  }
}

async function pullRemote() {
  try {
    isPulling.value = true;
    await invoke("pull_from_remote");
    await resfreshBranches();
  } catch (e) {
    console.log(e);
  } finally {
    isPulling.value = false;
  }
}
onMounted(() => {
  setInterval(async () => {
    await getPendingCommitsToPush();
    await getPendingCommitsToPull();
  }, 5000);
});

watchEffect(() => {
  if (repoStore.repo) {
    openRepo(repoStore.repo);
  }
});
</script>

<template>
  <nav
    class="sticky left-0 top-0 h-[100vh] w-64 flex flex-col text-white cursor-default"
  >
    <SearchBar />
    <h1 class="font-bold text-xl flex justify-center items-center gap-3 my-4">
      <v-icon name="pi-horsea" scale="1.5" /> Git Horse
    </h1>
    <h1 class="font-semibold">Current repository</h1>

    <span class="font-bold text-slate-200 text-lg">{{ repoName || "-" }}</span>
    <button
      class="text-black bg-slate-50 m-2 rounded-md font-bold hover:bg-slate-300 transition-colors duration-150 ease-in-out mx-4 p-1"
      @click="handleOpenFile"
    >
      <span class="flex relative gap-3">
        <i class="left-0">
          <v-icon fill="black" name="hi-folder-open" scale="1.2" />
        </i>
        <p>Open repo</p>
      </span>
    </button>
    <button
      class="text-black bg-slate-50 m-2 rounded-md font-bold hover:bg-slate-300 transition-colors duration-150 ease-in-out mx-4 p-1"
      @click="() => modalsStore.setSearchModalOpen(true)"
    >
      <span class="flex relative gap-3">
        <i class="left-0">
          <v-icon fill="black" name="hi-view-grid" scale="1.2" />
        </i>
        <p>Recent (Ctrl+K)</p>
      </span>
    </button>

    <div
      v-if="repoName"
      class="bg-[#605d63] m-2 rounded-md mx-4 flex flex-col overflow-auto h-[15rem]"
    >
      <h1 class="font-semibold text-left p-2 underline underline-offset-8">
        Active branches
      </h1>
      <div
        v-for="branch in localBranchesNames"
        :key="branch"
        class="text-left w-full text-black transition-colors duration-150 ease-in-out cursor-default font-semibold pl-2"
        as="button"
        :class="{
          'bg-[#d3ccdc]': branch === activeBranchName,
          'hover:text-slate-100': branch !== activeBranchName,
        }"
        @click="() => checkoutBranch(branch)"
      >
        {{ branch }}
      </div>
      <h1 class="font-semibold text-left p-2">Remote branches</h1>
      <div
        v-for="branch in remoteBranchesNames"
        :key="branch"
        class="text-left w-full text-black cursor-default font-semibold pl-2 flex justify-between pr-1"
      >
        {{ branch }}
        <strong
          class="hover:text-slate-300 transition-colors duration-150 ease-in-out cursor-default"
          ><v-icon name="hi-solid-chevron-double-right"
        /></strong>
      </div>
    </div>

    <button
      class="text-black bg-slate-50 m-2 rounded-md font-bold hover:bg-slate-300 transition-colors duration-150 ease-in-out mx-4 p-1"
      @click="fetchRemote"
    >
      <span class="flex relative gap-3">
        <i class="left-0">
          <v-icon fill="black" name="hi-cloud-download" scale="1.2" />
        </i>
        <p>Fetch</p>
        <aside v-if="isFetching" class="right-4 absolute">
          <v-icon name="si-spinrilla" class="animate-spin" />
        </aside>
      </span>
    </button>
    <button
      class="text-black bg-slate-50 m-2 rounded-md font-bold hover:bg-slate-300 transition-colors duration-150 ease-in-out mx-4 p-1"
      @click="pullRemote"
    >
      <span class="flex relative gap-3">
        <i class="left-0">
          <v-icon fill="black" name="hi-cloud-download" scale="1.2" />
        </i>
        <p>Pull</p>
        <aside v-if="pendingCommitsToPull">
          <v-icon name="bi-arrow-down-square" />

          {{ pendingCommitsToPull }}
        </aside>
        <aside v-if="isPulling" class="right-4 absolute">
          <v-icon name="si-spinrilla" class="animate-spin" />
        </aside>
      </span>
    </button>
    <button
      class="text-black bg-slate-50 m-2 rounded-md font-bold hover:bg-slate-300 transition-colors duration-150 ease-in-out mx-4 p-1"
      @click="pushRemote"
    >
      <span class="flex relative gap-3">
        <i class="left-0">
          <v-icon fill="black" name="hi-cloud-upload" scale="1.2" />
        </i>
        <p>Push</p>
        <aside v-if="pendingCommitsToPush">
          <v-icon name="bi-arrow-up-square" />

          {{ pendingCommitsToPush }}
        </aside>
        <aside v-if="isPushing" class="right-4 absolute">
          <v-icon name="si-spinrilla" class="animate-spin" />
        </aside>
      </span>
    </button>
  </nav>
</template>

<style scoped>
nav {
  background: #231e29;
}
</style>
