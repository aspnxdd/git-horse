<script setup lang="ts">
import type { Repos } from "src/shared/types";

import { invoke } from "@tauri-apps/api/tauri";
import { useRepoStore, useModalsStore } from "@stores";
import { debounce } from "src/shared/utils";

const DEBOUNCE_DELAY = 300;

const QUERIES = {
  ALL: "*",
  EMPTY: "",
} as const;

const allRepos = ref<Repos[]>([]);
const allReposFiltered = ref<Repos[]>([]);
const searchValue = ref<string | null>(null);
const search = ref<HTMLInputElement | null>(null);

const repoStore = useRepoStore();
const modalsStore = useModalsStore();

function queryFn(query: string) {
  if (query === QUERIES.EMPTY) {
    allReposFiltered.value = [];
    return;
  }
  if (query === QUERIES.ALL) {
    allReposFiltered.value = allRepos.value;
    return;
  }
  allReposFiltered.value = allRepos.value.filter((repo) =>
    repo.name.toLowerCase().includes(query.toLowerCase())
  );
  return;
}

const filterReposDebounced = debounce(queryFn, DEBOUNCE_DELAY);

function filterReposHandler(query: string) {
  searchValue.value = query;
  query === QUERIES.EMPTY || query === QUERIES.ALL
    ? queryFn(query)
    : filterReposDebounced(query);
}

function handleModal(e: MouseEvent) {
  if ((e.target as HTMLDivElement).nodeName === "DIV") {
    closeModal();
  }
}

function closeModal() {
  allReposFiltered.value = [];
  searchValue.value = null;
  modalsStore.setSearchModalOpen(false);
}

function selectRepo(path: string) {
  repoStore.setRepo(path);
  closeModal();
}

async function populateRepos() {
  const res = await invoke<Repos[]>("db_get_all");
  allRepos.value = res.filter((repo) => repo.name !== "last_opened_repo");
}

watchEffect(() => {
  if (search.value) {
    search.value.focus();
  }
});

onMounted(async () => {
  document.addEventListener("keydown", (e) => {
    if (e.code === "KeyK" && e.ctrlKey) {
      if (modalsStore.searchModal) {
        closeModal();
      } else {
        modalsStore.setSearchModalOpen(true);
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
      v-if="modalsStore.searchModal"
      class="flex w-full h-full fixed overflow-auto bg-slate-900 bg-opacity-60 text-black z-10"
      @click="handleModal"
    >
      <section
        class="bg-white absolute flex justify-center flex-col items-center w-2/4 rounded-xl shadow-xl p-1 top-10 left-1/4"
      >
        <span class="flex justify-center items-center w-full h-full">
          <v-icon
            fill="black"
            name="hi-search"
            scale="1.3"
            class="border-0 ml-6"
          />

          <input
            id="search"
            ref="search"
            autofocus
            placeholder="Type * to show all repos..."
            class="w-full h-full p-5 text-xl outline-white"
            @input="(e)=>filterReposHandler((e.target as HTMLInputElement).value)"
          />
          <p
            v-if="!searchValue"
            class="rounded-lg p-2 bg-zinc-200 border border-black"
          >
            Ctrl
          </p>
          <p
            v-if="!searchValue"
            class="rounded-lg p-2 bg-zinc-200 border border-black mx-2"
          >
            K
          </p>
        </span>

        <li
          v-for="repo in allReposFiltered"
          :key="repo.name"
          class="text-xl border-b-2 border-gray-400 p-3 w-full h-full flex justify-center items-center bg-white overflow-hidden search-results cursor-pointer hover:bg-slate-100"
          @click="() => selectRepo(repo.path)"
        >
          {{ repo.name }}
        </li>
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
