<script setup lang="ts">
import { useRepoStore, useModalsStore, useThemeStore } from "@stores";
import { open } from "@tauri-apps/api/dialog";
import { addNewRepoToDb, setLastOpenedRepoToDb } from "src/adapter/db";
import {
  findBranches,
  getCurrentBranchName,
  getPendingCommitsToPush,
  getRepoName,
  openRepo,
  getPendingCommitsToPull,
  checkoutBranch,
  getRemotes,
  fetchRemote,
  pushRemote,
  pullRemote,
  checkoutRemoteBranch,
} from "src/adapter/git-actions";

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

async function handleOpenRepo(path: string | null) {
  if (!path) {
    return;
  }
  await openRepo(path);
  const [_repoName, _repoPath] = await getRepoName();
  repoName.value = _repoName;
  await addNewRepoToDb(repoName.value, _repoPath);
  repoStore.setRepo(_repoPath);
  await resfreshBranches();
  await setLastOpenedRepoToDb(_repoPath);
  await handleGetPendingCommitsToPush();
  await handleGetPendingCommitsToPull();
}

async function handleOpenFile() {
  const file = await open({
    directory: true,
    multiple: false,
  });
  if (file && !Array.isArray(file)) {
    handleOpenRepo(file);
  }
}

watch(repoStore, async () => {
  await handleOpenRepo(repoStore.repo);
  await handleGetPendingCommitsToPush();
  await handleGetPendingCommitsToPull();
});

async function resfreshBranches() {
  localBranchesNames.value = await findBranches("Local");
  remoteBranchesNames.value = await findBranches("Remote");
  activeBranchName.value = await getCurrentBranchName();
  repoStore.setActiveBranch(activeBranchName.value);
}

async function handleGetPendingCommitsToPush() {
  try {
    const _pendingCommitsToPush = await getPendingCommitsToPush();
    pendingCommitsToPush.value = _pendingCommitsToPush;
  } catch (e) {
    pendingCommitsToPush.value = null;
  }
}

async function handleGetPendingCommitsToPull() {
  try {
    const _pendingCommitsToPull = await getPendingCommitsToPull();
    pendingCommitsToPull.value = _pendingCommitsToPull;
  } catch (e) {
    pendingCommitsToPull.value = null;
  }
}

async function handleCheckoutBranch(branch: string) {
  await checkoutBranch(branch);
  activeBranchName.value = branch;
  repoStore.setActiveBranch(activeBranchName.value);
}

async function handleCheckoutRemoteBranch(branch: string) {
  await checkoutRemoteBranch(branch);
  activeBranchName.value = branch;
  repoStore.setActiveBranch(activeBranchName.value);
}

async function handleGetRemotes() {
  console.log("remotes:", await getRemotes());
}

async function handleFetchRemote() {
  try {
    isFetching.value = true;
    await fetchRemote();
    await resfreshBranches();
  } finally {
    isFetching.value = false;
  }
}
async function handlePushRemote() {
  try {
    isPushing.value = true;
    await pushRemote();
    await resfreshBranches();
  } finally {
    isPushing.value = false;
  }
}

async function handlePullRemote() {
  try {
    isPulling.value = true;
    await pullRemote();
    await resfreshBranches();
  } finally {
    isPulling.value = false;
  }
}
onMounted(() => {
  setInterval(async () => {
    await handleGetPendingCommitsToPush();
    await handleGetPendingCommitsToPull();
  }, 5000);
});

watchEffect(() => {
  if (repoStore.repo) {
    handleOpenRepo(repoStore.repo);
  }
});
</script>

<template>
  <nav
    class="bg-background sticky left-0 top-0 h-[100vh] flex flex-col text-text cursor-default"
  >
    <h1 class="flex items-center justify-center gap-3 my-4 text-xl font-bold">
      <v-icon name="pi-horsea" scale="1.5" /> Git Horse
    </h1>
    <div class="flex flex-col items-center gap-2">
      <h1 class="font-semibold">Current repository</h1>

      <span class="text-lg font-bold text-text mb-7">{{
        repoName || "-"
      }}</span>
    </div>
    <button
      class="p-1 py-2 m-2 mx-4 font-bold transition-colors duration-150 ease-in-out rounded-md text-text hover:bg-primary"
      @click="handleOpenFile"
    >
      <span class="relative flex gap-3">
        <i class="left-0">
          <v-icon :fill="iconColor" name="hi-folder-open" scale="1.2" />
        </i>
        <p>Open repo</p>
      </span>
    </button>
    <button
      class="p-1 py-2 m-2 mx-4 font-bold transition-colors duration-150 ease-in-out rounded-md text-text hover:bg-primary"
      @click="() => modalsStore.setSearchModalOpen(true)"
    >
      <span class="relative flex gap-3">
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
      <h1 class="p-2 text-lg font-semibold text-left">Active branches</h1>
      <div
        v-for="branch in localBranchesNames"
        :key="branch"
        class="w-full py-1 pl-2 font-semibold text-left transition-colors duration-150 ease-in-out rounded-md cursor-default text-text"
        as="button"
        :class="{
          'bg-primary': branch === activeBranchName,
          'hover:text-text-hover': branch !== activeBranchName,
        }"
        @click="() => handleCheckoutBranch(branch)"
      >
        {{ branch }}
      </div>
      <h1 class="p-2 text-lg font-semibold text-left">Remote branches</h1>
      <div
        v-for="branch in remoteBranchesNames"
        :key="branch"
        class="flex justify-between w-full py-1 pl-2 pr-1 font-semibold text-left cursor-default text-text"
      >
        {{ branch }}
        <div>
          <v-icon
            name="hi-solid-chevron-double-right"
            as="button"
            class="transition-colors duration-150 ease-in-out cursor-default hover:text-text-hover"
            @click="() => handleCheckoutRemoteBranch(branch)"
          />
        </div>
      </div>
    </div>

    <button
      class="p-1 py-2 m-2 mx-4 font-bold transition-colors duration-150 ease-in-out rounded-md text-text hover:bg-primary"
      @click="handleFetchRemote"
    >
      <span class="relative flex gap-3">
        <i class="left-0">
          <v-icon :fill="iconColor" name="hi-cloud-download" scale="1.2" />
        </i>
        <p>Fetch</p>
        <aside v-if="isFetching" class="absolute right-4">
          <v-icon name="si-spinrilla" class="animate-spin" />
        </aside>
      </span>
    </button>
    <button
      class="p-1 py-2 m-2 mx-4 font-bold transition-colors duration-150 ease-in-out rounded-md text-text hover:bg-primary"
      @click="handlePullRemote"
    >
      <span class="relative flex gap-3">
        <i class="left-0">
          <v-icon :fill="iconColor" name="hi-cloud-download" scale="1.2" />
        </i>
        <p>Pull</p>
        <aside v-if="pendingCommitsToPull">
          <v-icon name="bi-arrow-down-square" />

          {{ pendingCommitsToPull }}
        </aside>
        <aside v-if="isPulling" class="absolute right-4">
          <v-icon name="si-spinrilla" class="animate-spin" />
        </aside>
      </span>
    </button>
    <button
      class="p-1 py-2 m-2 mx-4 font-bold transition-colors duration-150 ease-in-out rounded-md text-text hover:bg-primary"
      @click="handlePushRemote"
    >
      <span class="relative flex gap-3">
        <i class="left-0">
          <v-icon :fill="iconColor" name="hi-cloud-upload" scale="1.2" />
        </i>
        <p>Push</p>
        <aside v-if="pendingCommitsToPush">
          <v-icon name="bi-arrow-up-square" />

          {{ pendingCommitsToPush }}
        </aside>
        <aside v-if="isPushing" class="absolute right-4">
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
      <span class="relative flex gap-3">
        <i class="left-0">
          <v-icon :fill="iconColor" name="bi-palette" scale="1.2" />
        </i>
        <p>Change Theme</p>
      </span>
    </button>
  </nav>
</template>
