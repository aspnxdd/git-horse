<script setup lang="ts">
import { SideBar, FilesView } from "@components/Layout";
import { invoke } from "@tauri-apps/api/tauri";
import { useRepoStore } from "@stores";

const repoStore = useRepoStore();

onMounted(async () => {
  const res = await invoke<string>("read_last_opened_repo");
  console.log("res:", res);
  repoStore.setRepo(res);
});
</script>

<template>
  <div class="flex">
    <SideBar />
    <FilesView />
  </div>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
</style>
