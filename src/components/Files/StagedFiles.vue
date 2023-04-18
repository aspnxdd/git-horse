<script setup lang="ts">
import type { FileStatusWithStatusLabel, GitDiff } from "src/shared/types";

import { useRepoStore } from "@stores";
import { invoke } from "@tauri-apps/api/tauri";
import { FileEntry } from "./index";

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

function getModifiedFiles() {
  emits("getModifiedFiles");
}

const repoStore = useRepoStore();
const commitMessage = ref<string>("");

function displayFileDiffStaged(index: number) {
  if (props.filesStaged.length === 0) {
    return;
  }
  repoStore.setSelectedFile(props.filesStaged[index].fileName);
}

async function commit() {
  if (!commitMessage.value) {
    alert("Please enter commit message");
    return;
  }
  await invoke("commit", { message: commitMessage.value });
  getStagedFiles();
}
</script>

<template>
  <section
    v-if="filesStaged.length > 0"
    class="flex flex-col items-start w-2/5"
  >
    <span class="flex items-center justify-center gap-2 p-2">
      <h1 class="font-bold text-lg">Staged changes:</h1>
    </span>
    <ul
      class="list-none p-1 bg-[#4c4653] rounded-xl m-2 h-28 min-w-[20rem] text-xs overflow-y-scroll resize-y"
    >
      <li
        v-for="(stagedFileName, idx) in filesStaged"
        :key="stagedFileName.fileName"
        class="text-left p-1"
        @click="
          () => {
            displayFileDiffStaged(idx);
          }
        "
      >
        <FileEntry
          :file-name="stagedFileName.fileName"
          :status="stagedFileName.status"
          :is-input="false"
        />
      </li>
    </ul>
    <div class="relative flex justify-start flex-col w-[21rem] items-center">
      <textarea
        v-model="commitMessage"
        type="text"
        :class="[
          'rounded-lg  my-2 p-1 text-black h-40 text-left text-clip w-full text-sm border-2',
          commitMessage && commitMessage.length > 50
            ? 'border-yellow-500  outline-yellow-500'
            : 'border-transparent',
        ]"
        placeholder="Commit message"
      />
      <span
        v-if="commitMessage && commitMessage.length > 50"
        class="text-xs absolute text-red-500 rounded-b-lg bg-yellow-500 p-1 w-full bottom-8"
      >
        <strong>Your commit message should be less than 50 characters.</strong>
      </span>
      <button
        class="w-[50%] font-bold text-black bg-slate-50 rounded-md hover:bg-slate-300 transition-colors duration-150 ease-in-out"
        @click="commit"
      >
        Commit to {{ repoStore.activeBranch }}
      </button>
    </div>
  </section>
</template>
