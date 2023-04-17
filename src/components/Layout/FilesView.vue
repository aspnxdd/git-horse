<script setup lang="ts">
import type {
  FileStatusWithStatusLabel,
  GitDiff,
  FileStatus,
} from "src/shared/types";

import { useRepoStore } from "@stores";
import { invoke } from "@tauri-apps/api/tauri";
import { ChangedFiles, StagedFiles } from "@components/Files";
import FileDiff from "../Files/FileDiff.vue";
import { GitStatus, GitStatusCodes } from "src/shared/constants";

type RepoDiffStats = {
  deletions: number;
  filesChanged: number;
  insertions: number;
};

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
  const res = await invoke<GitDiff[]>("git_diff");
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

async function getModifiedFiles() {
  const modifiedFiles = await invoke<FileStatus[]>("get_modified_files");
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
  await getRepoDiff();
}

async function getStagedFiles() {
  const _stagedFilesNames = await invoke<FileStatus[]>("get_staged_files");
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
}

async function getRepoDiff() {
  repoDiffStats.value = await invoke<RepoDiffStats>("get_repo_diff");
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
  await getModifiedFiles();
  await getStagedFiles();
  await gitDiff();
});

onMounted(() => {
  setInterval(async () => {
    await getModifiedFiles();
    await getStagedFiles();
    await gitDiff();
  }, 5000);
});
</script>

<template>
  <main
    v-if="!repoStore.repo"
    class="flex flex-col items-center justify-center w-full p-4 text-slate-100"
  >
    <h1 class="text-2xl">Select a repository</h1>
  </main>
  <main
    v-else-if="filesModified.length === 0 && filesStaged.length === 0"
    class="flex flex-col items-center justify-center w-full p-4 text-slate-100"
  >
    <h1 class="text-2xl">No new changes</h1>
  </main>
  <main v-else class="w-full p-4 text-slate-100">
    <div class="flex flex-wrap w-full gap-3">
      <ChangedFiles
        :files-modified="filesModified"
        :is-all-files-changed-checked="isAllFilesChangedChecked"
        :repo-diff-lines="repoDiffLines"
        :repo-diff-stats="repoDiffStats"
        @update-files-modified-selection="updateFilesModifiedSelection"
        @toggle-all="toggleAll"
        @get-modified-files="getModifiedFiles"
        @get-staged-files="getStagedFiles"
      />
      <StagedFiles
        :files-staged="filesStaged"
        :repo-diff-lines="repoDiffLines"
        :repo-diff-stats="repoDiffStats"
        @get-staged-files="getStagedFiles"
        @get-modified-files="getModifiedFiles"
      />
    </div>

    <hr class="border-0 h-4" />

    <FileDiff
      :repo-diff-lines="repoDiffLines"
      :files-modified-names="[...filesStaged, ...filesModified]"
    />
  </main>
</template>

<style scoped>
main {
  background: #2e2a33;
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
