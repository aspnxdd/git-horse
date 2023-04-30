<script setup lang="ts">
import type { Repos } from "src/shared/types";

import { invoke } from "@tauri-apps/api/tauri";
import { useModalsStore, useThemeStore } from "@stores";
import { FunctionHighlighter } from "../Theme";

const allRepos = ref<Repos[]>([]);
const allReposFiltered = ref<Repos[]>([]);
const searchValue = ref<string | null>(null);

const modalsStore = useModalsStore();
const themeStore = useThemeStore();

const THEMES = ["Github Dimmed", "Github Light"];

function handleModal(e: MouseEvent) {
  if ((e.target as HTMLDivElement).nodeName === "DIV") {
    closeModal();
  }
}

function closeModal() {
  allReposFiltered.value = [];
  searchValue.value = null;
  modalsStore.setThemeModalOpen(false);
}

function switchTheme(theme: string) {
  themeStore.setTheme(theme.toLowerCase().replace(" ", "-"));
  modalsStore.setThemeModalOpen(false);
}

async function populateRepos() {
  const res = await invoke<Repos[]>("db_get_all");
  allRepos.value = res.filter((repo) => repo.name !== "last_opened_repo");
}

onMounted(async () => {
  document.addEventListener("keydown", (e) => {
    if (e.code === "Escape") {
      closeModal();
    }
  });
  await populateRepos();
});

onUpdated(async () => {
  await populateRepos();
});
</script>

<template>
  <Transition name="fade">
    <div
      v-if="modalsStore.themeModal"
      class="fixed z-10 flex w-full h-full overflow-auto text-black bg-slate-900 bg-opacity-60"
      @click="handleModal"
    >
      <div
        class="absolute flex flex-wrap items-center justify-center w-1/2 max-w-[35rem] gap-10 p-1 shadow-xl bg-slate-200 rounded-xl top-10 left-1/4"
      >
        <figure
          v-for="theme of THEMES"
          :key="theme"
          class="flex flex-col items-center justify-center p-3 transition-transform duration-150 cursor-pointer hover:scale-105 rounded-xl"
          @click="switchTheme(theme)"
        >
          <div
            class="p-4 rounded-xl"
            :style="{
              backgroundColor: theme === 'Github Light' ? '#fff' : '#24292e',
              color: theme === 'Github Light' ? '#22272e' : '#fff',
            }"
          >
            <function-highlighter :theme="theme" />
          </div>

          <figcaption class="mt-2 font-bold text-center">
            {{ theme }}
          </figcaption>
        </figure>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.search-results:first-of-type {
  border-top: 2px solid #9ca3af;
}
.search-results:last-of-type {
  border-bottom: 0;
}
.modal-content {
  padding: 2rem;
  padding-top: 4rem;
  background-color: #fefefe;
  margin: auto;
  overflow: hidden;
  display: flex;
  position: relative;
  flex-direction: column;
  width: max(40%, 40rem);
  border-radius: 1rem;
  height: max(30%, 10rem);
  box-shadow: 5px 5px 5px rgb(0 0 0 / 0.3);
}
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
