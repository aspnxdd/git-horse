<script setup lang="ts">
import type { FileStatusWithStatusLabel, GitDiff } from "src/shared/types";

import { useRepoStore } from "@stores";
import { invoke } from "@tauri-apps/api/tauri";
import { FileEntry } from "./index";

const props = defineProps<{
  filesModified: FileStatusWithStatusLabel[];
  repoDiffLines: GitDiff[];
  repoDiffStats: {
    deletions: number;
    filesChanged: number;
    insertions: number;
  };
  isAllFilesChangedChecked: boolean;
}>();

interface Emits {
  (e: "toggleAll"): void;
  (e: "updateFilesModifiedSelection", newValue: boolean, index: number): void;
  (e: "getModifiedFiles"): Promise<void>;
  (e: "getStagedFiles"): Promise<void>;
}

const emits = defineEmits<Emits>();

function toggleAll() {
  emits("toggleAll");
}

function updateFilesModifiedSelection(newValue: boolean, index: number) {
  emits("updateFilesModifiedSelection", newValue, index);
}

async function getModifiedFiles() {
  emits("getModifiedFiles");
}

async function getStagedFiles() {
  emits("getStagedFiles");
}

const repoStore = useRepoStore();

function displayFileDiffModified(index: number) {
  if (props.filesModified.length === 0) {
    return;
  }
  repoStore.setSelectedFile(props.filesModified[index].fileName);
}

async function add() {
  if (props.isAllFilesChangedChecked) {
    await invoke("add_all");
  } else {
    const files = (props.filesModified ?? []).reduce(
      (acc, { fileName, selected }) => {
        return selected ? [...acc, fileName] : acc;
      },
      [] as string[]
    );

    await invoke("add", { files });
  }
  await getModifiedFiles();
  await getStagedFiles();
}

async function discard() {
  if (props.isAllFilesChangedChecked) {
    const files = (props.filesModified ?? []).map(({ fileName }) => fileName);
    await invoke("discard", { files });
  } else {
    const files = (props.filesModified ?? []).reduce(
      (acc, { fileName, selected }) => {
        return selected ? [...acc, fileName] : acc;
      },
      [] as string[]
    );

    await invoke("discard", { files });
  }
  await getModifiedFiles();
}

const selectedModifiedFilesAmount = computed(() => {
  return props.filesModified.filter((v) => v.selected).length;
});
</script>
<template>
  <section class="flex flex-col items-start">
    <span class="flex items-center justify-center gap-2 p-2">
      <input
        type="checkbox"
        class="accent-pink-500"
        :checked="props.isAllFilesChangedChecked"
        @click="toggleAll"
      />
      <h1 class="font-bold text-lg">
        Changed files ({{ filesModified.length }})
      </h1>
    </span>

    <ul
      class="list-none p-1 bg-[#4c4653] rounded-xl m-2 h-28 min-w-[20rem] text-xs overflow-auto resize-y"
    >
      <li
        v-for="(file, idx) in filesModified"
        :key="file.fileName + file.selected + file.status"
        class="text-left p-1"
      >
        <FileEntry
          :file-name="file.fileName"
          :status="file.status"
          :checked="file.selected"
          @update:checked="
            (checkedValue) => updateFilesModifiedSelection(checkedValue, idx)
          "
          @display="() => displayFileDiffModified(idx)"
        />
      </li>
    </ul>
    <div class="flex flex-col gap-1 text-left ml-4 my-3">
      <span> 🟢 Insertions: {{ repoDiffStats?.insertions }}</span>
      <span> 🔴 Deletions: {{ repoDiffStats?.deletions }}</span>
    </div>
    <div class="flex w-full justify-between px-2 items-center">
      <button
        :disabled="filesModified.every((v) => !v.selected)"
        class="action-button w-[9.5rem]"
        @click="add"
      >
        Add ({{ selectedModifiedFilesAmount }})
      </button>
      <button
        :disabled="filesModified.every((v) => !v.selected)"
        class="action-button w-[9.5rem]"
        @click="discard"
      >
        Discard ({{ selectedModifiedFilesAmount }})
      </button>
    </div>
  </section>
</template>
