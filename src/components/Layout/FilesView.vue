<script setup lang="ts">
import type {
  FileStatusWithStatusLabel,
  GitDiff,
  FileStatus,
} from "src/shared/types";

import { useRepoStore } from "@stores";
import { invoke } from "@tauri-apps/api/tauri";
import { FileView } from "./index";
import FileDiff from "../FileDiff.vue";
import { GitStatus, GitStatusCodes } from "src/shared/constants";

type RepoDiffStats = {
  deletions: number;
  filesChanged: number;
  insertions: number;
};

const repoStore = useRepoStore();
const filesModified = ref<FileStatusWithStatusLabel[]>([]);
const filesStaged = ref<FileStatusWithStatusLabel[]>([]);
const isAllFilesChangedChecked = ref<boolean>(false);
const commitMessage = ref<string | null>(null);
const repoDiffStats = ref<RepoDiffStats>({
  deletions: 0,
  filesChanged: 0,
  insertions: 0,
});
const repoDiffLines = ref<GitDiff[]>([]);
const selectedFile = ref<string | null>(null);

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
  filesModified.value = (filesModified.value ?? []).map((fileModified) => {
    return {
      ...fileModified,
      selected: !isAllFilesChangedChecked.value,
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

function displayFileDiffModified(index: number) {
  if (filesModified.value.length === 0) {
    return;
  }
  selectedFile.value = filesModified.value[index].fileName;
}

function displayFileDiffStaged(index: number) {
  if (filesStaged.value.length === 0) {
    return;
  }
  selectedFile.value = filesStaged.value[index].fileName;
}

async function add() {
  if (isAllFilesChangedChecked.value) {
    await invoke("add_all");
  } else {
    const files = (filesModified.value ?? []).reduce(
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
  if (isAllFilesChangedChecked.value) {
    const files = (filesModified.value ?? []).reduce((acc, { fileName }) => {
      return [...acc, fileName];
    }, [] as string[]);
    await invoke("discard", { files });
  } else {
    const files = (filesModified.value ?? []).reduce(
      (acc, { fileName, selected }) => {
        return selected ? [...acc, fileName] : acc;
      },
      [] as string[]
    );

    await invoke("discard", { files });
  }
  await getModifiedFiles();
  await getStagedFiles();
}

async function commit() {
  if (!commitMessage.value) {
    alert("Please enter commit message");
    return;
  }
  await invoke("commit", { message: commitMessage.value });
  await getStagedFiles();
  selectedFile.value = filesModified.value[0]?.fileName;
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
      <section
        v-if="filesModified.length > 0"
        class="flex flex-col items-start"
      >
        <span class="flex items-center justify-center gap-2 p-2">
          <input
            type="checkbox"
            class="accent-pink-500"
            :checked="isAllFilesChangedChecked"
            @click="toggleAll"
            @change="
              () => (isAllFilesChangedChecked = !isAllFilesChangedChecked)
            "
          />
          <h1 class="font-bold text-lg">
            Files changed ({{ filesModified?.length }})
          </h1>
        </span>
        <div class="flex flex-col text-left ml-4">
          <span> 🟢 Insertions: {{ repoDiffStats?.insertions }}</span>
          <span> 🔴 Deletions: {{ repoDiffStats?.deletions }}</span>
        </div>
        <ul
          class="list-none p-1 bg-[#4c4653] rounded-xl m-2 h-28 min-w-[20rem] text-xs overflow-y-scroll resize-y"
        >
          <li
            v-for="(file, idx) in filesModified"
            :key="file.fileName + file.selected + file.status"
            class="text-left p-1"
          >
            <FileView
              :file-name="file.fileName"
              :status="file.status"
              :checked="file.selected"
              @update:checked="
                (checkedValue) =>
                  updateFilesModifiedSelection(checkedValue, idx)
              "
              @display="() => displayFileDiffModified(idx)"
            />
          </li>
        </ul>
        <div class="flex gap-4">
          <button
            :disabled="filesModified.every((v) => !v.selected)"
            class="px-4 ml-3 font-bold disabled:hover:bg-slate-400 disabled:bg-slate-400 text-black bg-slate-50 rounded-md hover:bg-slate-300 transition-colors duration-150 ease-in-out"
            @click="add"
          >
            Add ({{ filesModified.filter((v) => v.selected).length }})
          </button>
          <button
            :disabled="filesModified.every((v) => !v.selected)"
            class="px-4 ml-3 font-bold disabled:hover:bg-slate-400 disabled:bg-slate-400 text-black bg-slate-50 rounded-md hover:bg-slate-300 transition-colors duration-150 ease-in-out"
            @click="discard"
          >
            Discard ({{ filesModified.filter((v) => v.selected).length }})
          </button>
        </div>
      </section>
      <section
        v-if="filesStaged.length > 0"
        class="flex flex-col items-start w-2/5"
      >
        <span class="flex items-center justify-center gap-2 p-2">
          <h1 class="font-bold text-lg">Staged changes:</h1>
        </span>
        <ul class="list-none space-y-2">
          <li
            v-for="(stagedFileName, idx) in filesStaged"
            :key="stagedFileName.fileName"
            class="text-left p-2 text-xs bg-[#21325a] rounded-xl m-2"
            @click="
              () => {
                displayFileDiffStaged(idx);
              }
            "
          >
            <FileView
              :file-name="stagedFileName.fileName"
              :status="stagedFileName.status"
              :is-input="false"
            />
          </li>
        </ul>
        <textarea
          type="text"
          class="rounded-lg my-2 p-1 text-black h-40 text-left text-clip w-full text-sm"
          placeholder="Commit message"
          @change="(e)=>commitMessage=(e.target as HTMLTextAreaElement).value"
        />
        <button
          class="px-4 font-bold text-black bg-slate-50 rounded-md hover:bg-slate-300 transition-colors duration-150 ease-in-out"
          @click="commit"
        >
          Commit to {{ repoStore.activeBranch }}
        </button>
      </section>
    </div>

    <hr class="border-0 h-4" />

    <FileDiff
      :repo-diff-lines="repoDiffLines"
      :files-modified-names="[...filesStaged, ...filesModified]"
      :selected-file="selectedFile"
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
