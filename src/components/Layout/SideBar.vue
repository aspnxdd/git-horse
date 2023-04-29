<script setup lang="ts">
import { useRepoStore, useModalsStore, useThemeStore } from "@stores";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

const activeBranchName = ref<null | string>(null);
const repoName = ref<null | string>(null);
const localBranchesNames = ref<null | string[]>(null);
const remoteBranchesNames = ref<null | string[]>(null);
const repoStore = useRepoStore();
const modalsStore = useModalsStore();
const themeStore = useThemeStore();

const pendingCommitsToPush = ref<null | number>(null);
const pendingCommitsToPull = ref<null | number>(null);

const isFetching = ref(false);
const isPulling = ref(false);
const isPushing = ref(false);

const iconColor = computed(() => {
  if (themeStore.theme === "github-light") {
    return "#22272e";
  }
  return "#fff";
});

async function openRepo(path: string | null) {
  if (!path) {
    return;
  }
  await invoke("open", { path });
  const [_repoName, _repoPath] = await invoke<[string, string]>(
    "get_repo_name"
  );
  repoName.value = _repoName;
  await invoke("db_insert", {
    key: repoName.value,
    value: _repoPath,
  });
  repoStore.setRepo(_repoPath);
  await resfreshBranches();
  await invoke("write_last_opened_repo", {
    key: _repoPath,
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
    class="bg-background sticky left-0 top-0 h-[100vh] max-w-[256px]  min-w-[256px] flex flex-col text-text cursor-default border-r border-gray-500"
  >
    <h1 class="font-bold text-xl flex justify-center items-center gap-3 my-4">
      <v-icon name="pi-horsea" scale="1.5" /> Git Horse
    </h1>
    <div class="flex flex-col items-center gap-2">
      <h1 class="font-semibold">Current repository</h1>

      <span class="font-bold text-text text-lg mb-7">{{
        repoName || "-"
      }}</span>
    </div>
    <button
      class="text-text m-2 py-2 rounded-md font-bold hover:bg-primary transition-colors duration-150 ease-in-out mx-4 p-1"
      @click="handleOpenFile"
    >
      <span class="flex relative gap-3">
        <i class="left-0">
          <v-icon :fill="iconColor" name="hi-folder-open" scale="1.2" />
        </i>
        <p>Open repo</p>
      </span>
    </button>
    <button
      class="text-text m-2 py-2 rounded-md font-bold hover:bg-primary transition-colors duration-150 ease-in-out mx-4 p-1"
      @click="() => modalsStore.setSearchModalOpen(true)"
    >
      <span class="flex relative gap-3">
        <i class="left-0">
          <v-icon :fill="iconColor" name="hi-view-grid" scale="1.2" />
        </i>
        <p>Recent (Ctrl+K)</p>
      </span>
    </button>

    <div
      v-if="repoName"
      class="border-transparent border-2 m-2 rounded-md mx-4 flex flex-col overflow-auto h-[15rem]"
    >
      <h1 class="font-semibold text-left p-2 text-lg">Active branches</h1>
      <div
        v-for="branch in localBranchesNames"
        :key="branch"
        class="text-left py-1 rounded-md w-full text-text transition-colors duration-150 ease-in-out cursor-default font-semibold pl-2"
        as="button"
        :class="{
          'bg-primary': branch === activeBranchName,
          'hover:text-text-hover': branch !== activeBranchName,
        }"
        @click="() => checkoutBranch(branch)"
      >
        {{ branch }}
      </div>
      <h1 class="font-semibold text-left p-2 text-lg">Remote branches</h1>
      <div
        v-for="branch in remoteBranchesNames"
        :key="branch"
        class="text-left py-1 w-full text-text cursor-default font-semibold pl-2 flex justify-between pr-1"
      >
        {{ branch }}
        <strong
          class="hover:text-text-hover transition-colors duration-150 ease-in-out cursor-default"
          ><v-icon name="hi-solid-chevron-double-right"
        /></strong>
      </div>
    </div>

    <button
      class="text-text m-2 py-2 rounded-md font-bold hover:bg-primary transition-colors duration-150 ease-in-out mx-4 p-1"
      @click="fetchRemote"
    >
      <span class="flex relative gap-3">
        <i class="left-0">
          <v-icon :fill="iconColor" name="hi-cloud-download" scale="1.2" />
        </i>
        <p>Fetch</p>
        <aside v-if="isFetching" class="right-4 absolute">
          <v-icon name="si-spinrilla" class="animate-spin" />
        </aside>
      </span>
    </button>
    <button
      class="text-text m-2 py-2 rounded-md font-bold hover:bg-primary transition-colors duration-150 ease-in-out mx-4 p-1"
      @click="pullRemote"
    >
      <span class="flex relative gap-3">
        <i class="left-0">
          <v-icon :fill="iconColor" name="hi-cloud-download" scale="1.2" />
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
      class="text-text m-2 py-2 rounded-md font-bold hover:bg-primary transition-colors duration-150 ease-in-out mx-4 p-1"
      @click="pushRemote"
    >
      <span class="flex relative gap-3">
        <i class="left-0">
          <v-icon :fill="iconColor" name="hi-cloud-upload" scale="1.2" />
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
    <button
      class="text-text absolute bottom-4 py-2 left-1/2 w-[75%] -translate-x-1/2 rounded-md font-bold hover:bg-primary transition-colors duration-150 ease-in-out p-1"
      @click="
        () => {
          modalsStore.setThemeModalOpen(true);
        }
      "
    >
      <span class="flex relative gap-3">
        <i class="left-0">
          <v-icon :fill="iconColor" name="bi-palette" scale="1.2" />
        </i>
        <p>Change Theme</p>
      </span>
    </button>
  </nav>
</template>
