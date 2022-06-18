<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { File } from "./index";
import { useRepoStore } from "../../stores";

interface RepoDiffStats {
  deletions: number;
  files_changed: number;
  insertions: number;
}
const repoStore = useRepoStore();
const filesModifiedNames = ref<null | string[]>(null);
const checkboxIter = ref<boolean[]>([]);
const filesChangedToogle = ref<boolean>(true);
const repoDiffStats = ref<null | RepoDiffStats>(null);

async function getModifiedFiles() {
  filesModifiedNames.value = await invoke("get_modified_files");
  console.log("filesModifiedNames", filesModifiedNames.value);
}
async function getRepoDiff() {
  repoDiffStats.value = await invoke<RepoDiffStats>("get_repo_diff");
}
watch(repoStore, async () => {
  await getModifiedFiles();
  await getRepoDiff();
});

function toggleAll() {
  const falseArray = filesModifiedNames.value?.map(() => false) as boolean[];
  const trueArray = filesModifiedNames.value?.map(() => true) as boolean[];
  checkboxIter.value = filesChangedToogle.value ? falseArray : trueArray;
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
    v-else
    class="bg-[#0f172a] flex flex-col items-start w-full p-4 text-slate-100"
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
      <span> 🟠 Files changed: {{ repoDiffStats?.files_changed }}</span>
      <span> 🔴 Deletions: {{ repoDiffStats?.deletions }}</span>
    </div>
    <ul class="list-none">
      <li
        class="text-left p-2"
        v-for="(file, index) in filesModifiedNames"
        :key="file"
      >
        <File :file-name="file" :checked="checkboxIter[index]" />
      </li>
    </ul>
    <button
      class="px-4 font-bold text-black bg-slate-50 rounded-md hover:bg-slate-300 transition-colors duration-150 ease-in-out"
    >
      Add all
    </button>
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
