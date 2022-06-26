<script setup lang="ts">
import { useRepoStore } from "@stores";
import { invoke } from "@tauri-apps/api/tauri";
import { FileView } from "./index";
import { GitStatus, Replace, GitDiff, FileStatus } from "@types";
import FileDiff from "../FileDiff.vue";

interface RepoDiffStats {
  deletions: number;
  filesChanged: number;
  insertions: number;
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
const repoDiffLines = ref<GitDiff[]>([]);

watch(repoStore, async () => {
  await getModifiedFiles();
  await getStagedFiles();
  await gitDiff();
});

async function gitDiff() {
  const res = await invoke<GitDiff[]>("git_diff");
  repoDiffLines.value = res;
}
function getGitStatus(status: number) {
  if (status === 256) return GitStatus.Modified;
  if (status === 512) return GitStatus.Deleted;
  if (status === 128) return GitStatus.New;
  return GitStatus.Unknown;
}

async function getModifiedFiles() {
  const res = await invoke<FileStatus[]>("get_modified_files");
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
  stagedFilesNames.value = await invoke<string[]>("get_staged_files");
}
async function getRepoDiff() {
  repoDiffStats.value = await invoke<RepoDiffStats>("get_repo_diff");
}

function toggleAll() {
  const falseArray = filesModifiedNames.value?.map(() => false) as boolean[];
  const trueArray = filesModifiedNames.value?.map(() => true) as boolean[];
  checkboxIter.value = filesChangedToogle.value ? falseArray : trueArray;
}

function updateArr(b: boolean, index: number) {
  checkboxIter.value[index as number] = b;
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
      return checkboxIter.value[i as number] ? [...acc, fileName] : acc;
    }, [] as string[]);

    await invoke("add", { files });
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
}
onMounted(() => {
  setInterval(async () => {
    await getModifiedFiles();
    await getStagedFiles();
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
    v-else-if="filesModifiedNames.length == 0 && stagedFilesNames.length == 0"
    class="flex flex-col items-center justify-center w-full p-4 text-slate-100"
  >
    <h1 class="text-2xl">No new changes</h1>
  </main>
  <main v-else class="w-full p-4 text-slate-100">
    <div class="flex flex-wrap w-[50vw] gap-3">
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
            v-for="(file, index) in filesModifiedNames"
            :key="file.fileName"
            class="text-left p-2"
          >
            <FileView
              :file-name="file.fileName"
              :status="file.status"
              :checked="checkboxIter[index]"
              @update:checked="(b) => updateArr(b, index)"
            />
          </li>
        </ul>
        <button
          class="px-4 font-bold text-black bg-slate-50 rounded-md hover:bg-slate-300 transition-colors duration-150 ease-in-out"
          @click="add"
        >
          Add
        </button>
      </section>
      <hr class="border-0 h-4" />
      <section class="flex flex-col items-start w-2/5">
        <h1 class="font-bold text-lg">Staged changes:</h1>
        <ul
          v-if="stagedFilesNames.length > 0"
          class="list-none p-2 bg-[#21325a] rounded-xl m-2"
        >
          <li
            v-for="file in stagedFilesNames"
            :key="file"
            class="text-left p-2"
          >
            {{ file }}
          </li>
        </ul>
        <textarea
          type="text"
          class="rounded-lg my-2 p-1 text-black h-40 text-left text-clip w-full"
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
    <FileDiff
      :repo-diff-lines="repoDiffLines"
      :files-modified-names="filesModifiedNames"
    />
  </main>
</template>

<style scoped>
main {
  background: rgb(55, 55, 149);
  background: linear-gradient(
    120deg,
    rgba(55, 55, 149, 1) 0%,
    rgba(69, 123, 229, 1) 100%
  );
  cursor: default;
}
</style>
