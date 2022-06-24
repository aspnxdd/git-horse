<script setup lang="ts">
import { SideBar, FilesView } from "@components/Layout";
import { invoke } from "@tauri-apps/api/tauri";
import { useRepoStore } from "@stores";

const repoStore = useRepoStore();

onMounted(async () => {
  const res = await invoke<string>("read_last_opened_repo");
  repoStore.setRepo(res);
});
</script>

<template>
  <SideBar />
  <FilesView />
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  display: flex;
}
</style>
