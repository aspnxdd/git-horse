<script setup lang="ts">
import type {
  FileStatusWithStatusLabel,
  GitDiff,
  RepoDiffStats,
} from "src/shared/types";

import { useRepoStore } from "@stores";
import { ChangedFiles, StagedFiles } from "@components/Files";
import FileDiff from "../Files/FileDiff.vue";
import { GitStatus, GitStatusCodes } from "src/shared/constants";
import {
  getGitDiff,
  getModifiedFiles,
  getRepoDiff,
  getStagedFiles,
} from "src/adapter/git-actions";

const repoStore = useRepoStore();
const filesModified = ref<FileStatusWithStatusLabel[]>([]);
const filesStaged = ref<FileStatusWithStatusLabel[]>([]);
const repoDiffStats = ref<RepoDiffStats>({
  deletions: 0,
  filesChanged: 0,
  insertions: 0,
});
const repoDiffLines = ref<GitDiff[]>([]);
const selectedFile = ref<string | null>(null);
const isAllFilesChangedChecked = ref<boolean>(false);

async function gitDiff() {
  const res = await getGitDiff();
  repoDiffLines.value = res;
  if (selectedFile.value === null) {
    selectedFile.value = filesStaged.value[0]?.fileName;
  }
}

function getGitStatus(status: number) {
  if (Object.hasOwn(GitStatusCodes, status)) {
    return GitStatusCodes[status as keyof typeof GitStatusCodes];
  }
  return GitStatus.Unknown;
}

async function handleGetModifiedFiles() {
  const modifiedFiles = await getModifiedFiles();
  const fileStatuses = modifiedFiles.map(({ fileName, status }) => {
    return {
      fileName,
      status: getGitStatus(status),
      selected:
        filesModified.value.find((e) => e.fileName === fileName)?.selected ??
        false,
    };
  });
  filesModified.value = fileStatuses;
  await handleGetRepoDiff();
  if (!repoStore.selectedFile) {
    repoStore.setSelectedFile(filesModified.value[0]?.fileName);
  }
}

async function handleGetStagedFiles() {
  const _stagedFilesNames = await getStagedFiles();
  const fileStatuses = _stagedFilesNames.map(({ fileName, status }) => {
    return {
      fileName,
      selected:
        filesStaged.value.find((e) => e.fileName === fileName)?.selected ??
        false,
      status: getGitStatus(status),
    };
  });
  filesStaged.value = fileStatuses;

  if (!repoStore.selectedFile) {
    repoStore.setSelectedFile(filesStaged.value[0]?.fileName);
  }
}

async function handleGetRepoDiff() {
  repoDiffStats.value = await getRepoDiff();
}

function toggleAll() {
  isAllFilesChangedChecked.value = !isAllFilesChangedChecked.value;
  filesModified.value = (filesModified.value ?? []).map((fileModified) => {
    return {
      ...fileModified,
      selected: isAllFilesChangedChecked.value,
    };
  });
}

function updateFilesModifiedSelection(newValue: boolean, index: number) {
  filesModified.value[index].selected = newValue;
  isAllFilesChangedChecked.value = false;
  if (filesModified.value.every(({ selected }) => !!selected)) {
    isAllFilesChangedChecked.value = true;
  }
}

watch(repoStore, async () => {
  await handleGetModifiedFiles();
  await handleGetStagedFiles();
  await gitDiff();
});

onMounted(() => {
  setInterval(async () => {
    await handleGetModifiedFiles();
    await handleGetStagedFiles();
    await gitDiff();
  }, 5000);
});
</script>

<template>
  <main
    v-if="!repoStore.repo"
    class="flex flex-col items-center h-[100vh] justify-center p-4 text-text bg-background"
  >
    <h1 class="text-2xl">Select a repository</h1>
  </main>
  <main
    v-else-if="filesModified.length === 0 && filesStaged.length === 0"
    class="flex flex-col items-center justify-center p-4 text-text bg-background h-[100vh]"
  >
    <h1 class="text-2xl">No new changes</h1>
  </main>
  <main v-else class="p-4 text-text bg-background h-[100vh]">
    <div class="flex flex-wrap w-full gap-3">
      <ChangedFiles
        :files-modified="filesModified"
        :is-all-files-changed-checked="isAllFilesChangedChecked"
        :repo-diff-lines="repoDiffLines"
        :repo-diff-stats="repoDiffStats"
        @update-files-modified-selection="updateFilesModifiedSelection"
        @toggle-all="toggleAll"
        @get-modified-files="handleGetModifiedFiles"
        @get-staged-files="handleGetStagedFiles"
      />
      <StagedFiles
        :files-staged="filesStaged"
        :repo-diff-lines="repoDiffLines"
        :repo-diff-stats="repoDiffStats"
        @get-staged-files="handleGetStagedFiles"
        @get-modified-files="handleGetModifiedFiles"
      />
    </div>

    <hr class="h-4 border-0" />

    <FileDiff
      :repo-diff-lines="repoDiffLines"
      :files-modified-names="[...filesStaged, ...filesModified]"
    />
  </main>
</template>

<style scoped>
main {
  cursor: default;
}
.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>
