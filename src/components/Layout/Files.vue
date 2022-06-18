<script setup lang="ts">
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { File } from "./index";

const filesModifiedNames = ref<null | string[]>(null);
const checked = ref<boolean[]>([]);
const filesChangedToogle = ref<boolean>(false);
async function getModifiedFiles() {
  filesModifiedNames.value = await invoke("get_modified_files");
  console.log("filesModifiedNames", filesModifiedNames.value);
}
onMounted(async () => {
  await getModifiedFiles();
});

function toggleAll() {
  const falseArray = filesModifiedNames.value?.map(() => false) as boolean[];
  const trueArray = filesModifiedNames.value?.map(() => true) as boolean[];
  checked.value = filesChangedToogle.value ? falseArray : trueArray;
}
</script>

<template>
  <main class="bg-[#0f172a] flex flex-col items-start w-full p-4">
    <span class="flex items-center justify-center gap-2 p-2">
      <input
        type="checkbox"
        class="accent-pink-500"
        @click="toggleAll"
        @change="() => (filesChangedToogle = !filesChangedToogle)"
      />
      <h1 class="font-bold text-lg">Files changed</h1>
    </span>

    <ul class="list-none">
      <li
        class="text-left p-2"
        v-for="(file, index) in filesModifiedNames"
        :key="file"
      >
        <File :file-name="file" :checked="checked[index]" />
      </li>
    </ul>
    <button
      class="px-4 font-bold bg-slate-50 rounded-md hover:bg-slate-300 transition-colors duration-150 ease-in-out"
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
