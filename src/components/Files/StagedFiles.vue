<script setup lang="ts">
import type { FileStatusWithStatusLabel, GitDiff } from "src/shared/types";

import { useRepoStore } from "@stores";
import { FileEntry } from "./index";
import { commit, unstageFile, unstageAll } from "src/adapter/git-actions";

const props = defineProps<{
  filesStaged: FileStatusWithStatusLabel[];
  repoDiffLines: GitDiff[];
  repoDiffStats: {
    deletions: number;
    filesChanged: number;
    insertions: number;
  };
}>();

interface Emits {
  (e: "getStagedFiles"): void;
  (e: "getModifiedFiles"): Promise<void>;
}

const emits = defineEmits<Emits>();

function getStagedFiles() {
  emits("getStagedFiles");
}

const repoStore = useRepoStore();
const commitMessage = ref<string>("");

function displayFileDiffStaged(index: number) {
  if (props.filesStaged.length === 0) {
    return;
  }
  repoStore.setSelectedFile(props.filesStaged[index].fileName);
}

async function handleCommit() {
  if (!commitMessage.value) {
    alert("Please enter commit message");
    return;
  }
  await commit(commitMessage.value);
  getStagedFiles();
}
</script>

<template>
  <section class="flex flex-col items-start w-2/5">
    <span class="flex items-center justify-center gap-2 p-2">
      <h1 class="text-lg font-bold">Staged changes:</h1>
    </span>
    <ul
      class="list-none p-1 bg-text-area-background rounded-xl m-2 h-28 w-[23rem] text-xs overflow-auto resize-y"
    >
      <li
        v-for="(stagedFileName, idx) in filesStaged"
        :key="stagedFileName.fileName"
        class="p-1 text-left"
        @click="() => displayFileDiffStaged(idx)"
      >
        <FileEntry
          :file-name="stagedFileName.fileName"
          :status="stagedFileName.status"
          :is-input="false"
        />
      </li>
    </ul>
    <button
      @click="
        () => {
          unstageFile(filesStaged[0].fileName);
        }
      "
    >
      Unstage file
    </button>
    <button
      @click="
        () => {
          unstageAll();
        }
      "
    >
      Unstage all
    </button>
    <div class="relative flex justify-start flex-col w-[21rem] items-center">
      <textarea
        v-model="commitMessage"
        type="text"
        :class="[
          'rounded-lg  my-2 p-1 text-black h-40 text-left text-clip w-[20rem] text-sm border-2',
          commitMessage && commitMessage.length > 50
            ? 'border-yellow-500  outline-yellow-500'
            : 'border-transparent',
        ]"
        placeholder="Commit message"
      />
      <span
        v-if="commitMessage && commitMessage.length > 50"
        class="absolute w-full p-1 text-xs text-red-500 bg-yellow-500 rounded-b-lg bottom-8"
      >
        <strong>Your commit message should be less than 50 characters.</strong>
      </span>
      <button class="action-button w-[95%]" @click="handleCommit">
        Commit to {{ repoStore.activeBranch }}
      </button>
    </div>
  </section>
</template>
