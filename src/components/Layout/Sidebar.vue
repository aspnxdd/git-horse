<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
const activeBranchName = ref<null | string>(null);
const repoName = ref<null | string>(null);
const localBranchesNames = ref<null | string[]>(null);
const remoteBranchesNames = ref<null | string[]>(null);
async function openRepo() {
  const selected = await open({
    directory: true,
    multiple: false,
  });
  await invoke("open", { path: selected });
  await resfreshBranches();
  repoName.value = await invoke("get_repo_name");
}
async function resfreshBranches() {
  localBranchesNames.value = await invoke("find_branches", { filter: "Local" });
  remoteBranchesNames.value = await invoke("find_branches", {
    filter: "Remote",
  });
  activeBranchName.value = await invoke("get_current_branch_name");
}
async function checkoutBranch(branch: string) {
  console.log("checkout", branch);
  await invoke("checkout_branch", { branchName: branch });
  console.log(123, await invoke("get_current_branch_name"));
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
    class="relative left-0 top-0 h-screen bg-blue-900 w-52 flex flex-col text-white"
  >
    <h1 class="font-semibold">Current repository</h1>
    <span class="font-semibold text-slate-400">{{ repoName || "-" }}</span>
    <button
      class="text-black bg-slate-50 m-2 rounded-md font-bold hover:bg-slate-300 transition-colors duration-150 ease-in-out"
      @click="openRepo"
    >
      Open repo
    </button>
    <button @click="getRemotes">Get remotes</button>
    <button @click="fetchRemote">Fetch remote</button>
    <div class="bg-blue-700 m-2 rounded-md" v-if="repoName">
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
            >>></strong
          >
        </div>
      </div>
    </div>
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
