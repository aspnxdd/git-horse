<!-- eslint-disable @typescript-eslint/no-non-null-assertion -->
<script setup lang="ts">
import { SideBar, FilesView } from "@components/Layout";
import { invoke } from "@tauri-apps/api/tauri";
import { useRepoStore, useThemeStore } from "@stores";
import { SearchBar, ThemeSelector } from "@components/Modal";

const repoStore = useRepoStore();
const themeStore = useThemeStore();

const html = document.querySelector("html")!;

onMounted(async () => {
  html.attributes.setNamedItem(document.createAttribute("data-theme"));
  html.attributes.getNamedItem("data-theme")!.value = themeStore.theme;
  const res = await invoke<string>("read_last_opened_repo");
  repoStore.setRepo(res);
});

watch(
  () => themeStore.theme,
  (theme) => {
    html.attributes.getNamedItem("data-theme")!.value = theme;
  }
);
</script>

<template>
  <SearchBar />
  <ThemeSelector />
  <SideBar />
  <FilesView />
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  display: flex;
}
</style>
