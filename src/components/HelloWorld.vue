<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
async function openDialog() {
  const selected = await open({
    directory: true,
    multiple: false,
  });
  console.log(selected);
  await invoke("open", { path: selected });
  await invoke("find_branches");
  const name = await invoke("get_current_branch_name");
  console.log(name);


}
defineProps<{ msg: string }>();

const count = ref(0);
async function backendAdd() {
  count.value = await invoke("backend_add", { number: count.value });
}
</script>

<template>
  <div>
    <button @click="openDialog">Open Dialog</button>
  </div>
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
