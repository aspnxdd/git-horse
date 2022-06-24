<script setup lang="ts">
import { useRepoStore } from "@stores";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { SearchBar } from "@components/Modal";

const activeBranchName = ref<null | string>(null);
const repoName = ref<null | string>(null);
const localBranchesNames = ref<null | string[]>(null);
const remoteBranchesNames = ref<null | string[]>(null);
const modalOpen = ref(false);
const repoStore = useRepoStore();

async function openRepo(path: string | null) {
  await invoke("open", { path });
  repoName.value = await invoke("get_repo_name");

  await invoke("db_insert", {
    key: repoName.value,
    value: path,
  });
  repoStore.setRepo(path as string);
  await resfreshBranches();
  await invoke("write_last_opened_repo", {
    key: path,
  });
}

async function handleOpenFile() {
  const file = (await open({
    directory: true,
    multiple: false,
  })) as string;
  console.log("file:", file);
  if (file) {
    openRepo(file);
  }
}

watch(repoStore, async () => {
  await openRepo(repoStore.repo);
});


async function resfreshBranches() {
  localBranchesNames.value = await invoke("find_branches", { filter: "Local" });
  remoteBranchesNames.value = await invoke("find_branches", {
    filter: "Remote",
  });
  activeBranchName.value = await invoke("get_current_branch_name");
  repoStore.setActiveBranch(activeBranchName.value as string);
}

async function checkoutBranch(branch: string) {
  await invoke("checkout_branch", { branchName: branch });
  activeBranchName.value = await invoke("get_current_branch_name");
}
async function getRemotes() {
  console.log("remotes:", await invoke("get_remotes"));
}
async function fetchRemote() {
  await invoke("fetch_remote");
  await resfreshBranches();
}

onMounted(() => {
  document.addEventListener("keydown", (e) => {
    if (e.code == "KeyK" && e.ctrlKey) {
      modalOpen.value = true;
    }
  });
});
</script>

<template>
  <nav
    class="relative left-0 top-0 h-screen w-60 flex flex-col text-white cursor-default"
  >
    <SearchBar :modal-open="modalOpen" @close:modal="modalOpen = false" />
    <h1 class="font-bold text-xl flex justify-center items-center gap-3 my-4">
      <v-icon name="gi-horse-head" scale="1.5" /> Git Horse
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
      @click="modalOpen = true"
    >
      <span class="flex relative gap-3">
        <i class="left-0">
          <v-icon fill="black" name="hi-view-grid" scale="1.2" />
        </i>
        <p>Recent</p>
      </span>
    </button>

    <div v-if="repoName" class="bg-indigo-500 m-2 rounded-md mx-4">
      <h1 class="font-semibold text-left p-2">Active branches</h1>
      <hr class="border-b-[1px] mx-4 mb-1" />

      <div
        v-for="branch in localBranchesNames"
        :key="branch"
        class="list-none text-left w-full"
      >
        <div
          as="button"
          class="text-black transition-colors duration-150 ease-in-out cursor-default font-semibold pl-2"
          :class="{
            'bg-pink-300': branch === activeBranchName,
            'hover:text-slate-100': branch !== activeBranchName,
          }"
          @click="checkoutBranch(branch)"
        >
          {{ branch }}
        </div>
      </div>
      <h1 class="font-semibold text-left p-2">Remote branches</h1>
      <div
        v-for="branch in remoteBranchesNames"
        :key="branch"
        class="list-none text-left w-full"
      >
        <div
          class="text-black cursor-default font-semibold pl-2 flex justify-between pr-1"
        >
          {{ branch }}
          <strong
            class="hover:text-slate-300 transition-colors duration-150 ease-in-out cursor-default"
            ><v-icon name="hi-solid-chevron-double-right"
          /></strong>
        </div>
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
      </span>
    </button>
    <button
      class="text-black bg-slate-50 m-2 rounded-md font-bold hover:bg-slate-300 transition-colors duration-150 ease-in-out mx-4 p-1"
      @click="fetchRemote"
    >
      <span class="flex relative gap-3">
        <i class="left-0">
          <v-icon fill="black" name="hi-cloud-upload" scale="1.2" />
        </i>
        <p>Push</p>
      </span>
    </button>
  </nav>
</template>

<style scoped>
nav {
  background: rgb(8, 8, 111);
  background: linear-gradient(
    45deg,
    rgba(8, 8, 111, 1) 0%,
    rgba(46, 107, 227, 1) 100%
  );
}
</style>
