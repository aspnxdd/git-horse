<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { File } from "./index";
import { useRepoStore } from "../../stores";
import { GitStatus, Replace } from "../../types";
interface RepoDiffStats {
  deletions: number;
  filesChanged: number;
  insertions: number;
}

interface FileStatus {
  fileName: string;
  status: number;
}

const repoStore = useRepoStore();
const filesModifiedNames = ref<Replace<FileStatus, "status", GitStatus>[]>([]);
const stagedFilesNames = ref<string[]>([]);
const checkboxIter = ref<boolean[]>([]);
const filesChangedToogle = ref<boolean>(true);
const commitMessage = ref<string | null>(null);
const repoDiffStats = ref<RepoDiffStats>({
  deletions: 0,
  filesChanged: 0,
  insertions: 0,
});

function getGitStatus(status: number) {
  if (status === 256) return GitStatus.Modified;
  if (status === 512) return GitStatus.Deleted;
  if (status === 128) return GitStatus.New;
  return GitStatus.Unknown;
}

async function getModifiedFiles() {
  const res = await invoke<FileStatus[]>("get_modified_files");
  console.log("get_modified_files", res);
  const fileStatuses = res.map((i) => {
    return {
      fileName: i.fileName,
      status: getGitStatus(i.status),
    };
  });
  filesModifiedNames.value = fileStatuses;
  console.log(123, fileStatuses);
  await getRepoDiff();
}
async function getStagedFiles() {
  const res = await invoke<string[]>("get_staged_files");
  console.log("get_staged_files", res);
  stagedFilesNames.value = res;
}
async function getRepoDiff() {
  repoDiffStats.value = await invoke<RepoDiffStats>("get_repo_diff");
}
watch(repoStore, async () => {
  await getModifiedFiles();
  await getStagedFiles();
});

function toggleAll() {
  const falseArray = filesModifiedNames.value?.map(() => false) as boolean[];
  const trueArray = filesModifiedNames.value?.map(() => true) as boolean[];
  checkboxIter.value = filesChangedToogle.value ? falseArray : trueArray;
}
function updateArr(b: boolean, index: number) {
  checkboxIter.value[index] = b;
  filesChangedToogle.value = false;
  if (checkboxIter.value.every((v) => v === true)) {
    filesChangedToogle.value = true;
  }
}

async function add() {
  if (filesChangedToogle.value) {
    await invoke("add_all");
  } else {
    const files = filesModifiedNames.value?.reduce((acc, { fileName }, i) => {
      const _acc = checkboxIter.value[i] ? [...acc, fileName] : acc;
      return _acc;
    }, [] as string[]);

    await invoke("add", { files });
  }
  await getModifiedFiles();
}
async function commit() {
  if (!commitMessage.value) {
    alert("Please enter commit message");
    return;
  }

  await invoke("commit", { message: commitMessage.value });
  await getStagedFiles();
}
</script>

<template>
  <main
    class="bg-[#0f172a] flex flex-col items-center justify-center w-full p-4 text-slate-100"
    v-if="!repoStore.repo"
  >
    <h1 class="text-2xl">Select a repository</h1>
  </main>
  <main
    class="bg-[#0f172a] flex flex-col items-center justify-center w-full p-4 text-slate-100"
    v-else-if="filesModifiedNames.length == 0 && stagedFilesNames.length == 0"
  >
    <h1 class="text-2xl">No new changes</h1>
  </main>
  <main v-else class="bg-[#0f172a] w-full p-4 text-slate-100">
    <section
      v-if="filesModifiedNames.length > 0"
      class="flex flex-col items-start"
    >
      <span class="flex items-center justify-center gap-2 p-2">
        <input
          type="checkbox"
          class="accent-pink-500"
          :checked="filesChangedToogle"
          @click="toggleAll"
          @change="() => (filesChangedToogle = !filesChangedToogle)"
        />
        <h1 class="font-bold text-lg">
          Files changed ({{ filesModifiedNames?.length }})
        </h1>
      </span>
      <div class="flex flex-col text-left ml-4">
        <span> 🟢 Insertions: {{ repoDiffStats?.insertions }}</span>
        <span> 🔴 Deletions: {{ repoDiffStats?.deletions }}</span>
      </div>
      <ul class="list-none p-2 bg-[#21325a] rounded-xl m-2">
        <li
          class="text-left p-2"
          v-for="(file, index) in filesModifiedNames"
          :key="file.fileName"
        >
          <File
            :file-name="file.fileName"
            :status="file.status"
            :checked="checkboxIter[index]"
            @update:checked="(b) => updateArr(b, index)"
          />
        </li>
      </ul>
      <button
        @click="add"
        class="px-4 font-bold text-black bg-slate-50 rounded-md hover:bg-slate-300 transition-colors duration-150 ease-in-out"
      >
        Add
      </button>
    </section>
    <hr class="border-0 h-4" />
    <section class="flex flex-col items-start">
      <h1 class="font-bold text-lg">Staged changes:</h1>
      <ul
        class="list-none p-2 bg-[#21325a] rounded-xl m-2"
        v-if="stagedFilesNames.length > 0"
      >
        <li
          class="text-left p-2"
          v-for="(file, index) in stagedFilesNames"
          :key="file"
        >
          {{ file }}
        </li>
      </ul>
      <input
        type="text"
        class="rounded-lg my-2 p-1 text-black"
        placeholder="Commit message"
        v-model="commitMessage"
      />
      <button
        @click="commit"
        class="px-4 font-bold text-black bg-slate-50 rounded-md hover:bg-slate-300 transition-colors duration-150 ease-in-out"
      >
        Commit to {{ repoStore.activeBranch }}
      </button>
    </section>
  </main>
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
