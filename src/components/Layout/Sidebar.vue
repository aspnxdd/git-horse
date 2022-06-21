<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { useRepoStore } from "../../stores/index";

const activeBranchName = ref<null | string>(null);
const repoName = ref<null | string>(null);
const localBranchesNames = ref<null | string[]>(null);
const remoteBranchesNames = ref<null | string[]>(null);

const repoStore = useRepoStore();
async function openRepo() {
  const selected = await open({
    directory: true,
    multiple: false,
  });
  await invoke("open", { path: selected });
  await resfreshBranches();
  repoName.value = await invoke("get_repo_name");

  repoStore.setRepo(repoName.value as string);
}
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
</script>

<template>
  <nav
    class="relative left-0 top-0 h-screen bg-blue-900 w-60 flex flex-col text-white"
  >
    <h1 class="font-bold text-xl flex justify-center items-center gap-3 my-4">
      <v-icon name="gi-horse-head" scale="1.5" /> Git Horse
    </h1>
    <h1 class="font-semibold">Current repository</h1>

    <span class="font-semibold text-slate-400">{{ repoName || "-" }}</span>
    <button
      class="text-black bg-slate-50 m-2 rounded-md font-bold hover:bg-slate-300 transition-colors duration-150 ease-in-out mx-4 p-1"
      @click="openRepo"
    >
      <span class="flex relative  gap-3">
        <i class="left-0">
          <v-icon fill="black" name="hi-folder-open" scale="1.2" />
        </i>
        <p>Open repo</p>
      </span>
    </button>
    <div class="bg-indigo-700 m-2 rounded-md mx-4" v-if="repoName">
      <h1 class="font-semibold text-left p-2">Active branches</h1>
      <hr class="border-b-[1px] mx-4 mb-1" />

      <div
        class="list-none text-left w-full"
        v-for="branch in localBranchesNames"
        :key="branch"
      >
        <div
          as="button"
          @click="checkoutBranch(branch)"
          class="text-black hover:text-slate-300 transition-colors duration-150 ease-in-out cursor-default font-semibold pl-2"
          :class="{ 'bg-sky-600': branch === activeBranchName }"
        >
          {{ branch }}
        </div>
      </div>
      <h1 class="font-semibold text-left p-2">Remote branches</h1>
      <div
        class="list-none text-left w-full"
        v-for="branch in remoteBranchesNames"
        :key="branch"
      >
        <div
          class="text-black cursor-default font-semibold pl-2 flex justify-between pr-1"
        >
          {{ branch }}
          <strong
            class="hover:text-slate-300 transition-colors duration-150 ease-in-out cursor-default"
            ><v-icon name="hi-solid-chevron-double-right" /></strong
          >
        </div>
      </div>
    </div>
    <button
      class="text-black bg-slate-50 m-2 rounded-md font-bold hover:bg-slate-300 transition-colors duration-150 ease-in-out mx-4 p-1"
      @click="fetchRemote"
    >
      <span class="flex relative  gap-3">
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
      <span class="flex relative  gap-3">
        <i class="left-0">
          <v-icon fill="black" name="hi-cloud-upload" scale="1.2" />
        </i>
        <p>Push</p>
      </span>
    </button>
  </nav>
</template>

<style scoped>
a {
  color: #42b983;
}

label {
  margin: 0 0.5em;
  font-weight: bold;
}

code {
  background-color: #eee;
  padding: 2px 4px;
  border-radius: 4px;
  color: #304455;
}
</style>
