<script setup lang="ts">
import type { Repos } from "src/shared/types";

import { invoke } from "@tauri-apps/api/tauri";
import { useModalsStore, useThemeStore } from "@stores";

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
    if (e.code === "KeyK" && e.ctrlKey) {
      if (modalsStore.themeModal) {
        closeModal();
      } else {
        modalsStore.setThemeModalOpen(true);
      }
    }
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
      class="flex w-full h-full fixed overflow-auto bg-slate-900 bg-opacity-60 text-black z-10"
      @click="handleModal"
    >
      <section
        class="bg-white absolute flex justify-center flex-col items-center w-2/4 rounded-xl shadow-xl p-1 top-10 left-1/4"
      >
        <div v-for="theme of THEMES" :key="theme">
          <button
            class="w-full text-left p-2 hover:bg-gray-200"
            @click="() => switchTheme(theme)"
          >
            {{ theme }}
          </button>
        </div>
      </section>
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
