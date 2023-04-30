<script setup lang="ts">
import type { Repos } from "src/shared/types";

import { useRepoStore, useModalsStore } from "@stores";
import { debounce } from "src/shared/utils";
import { getAllReposFromDb } from "src/adapter/db";

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
  const res = await getAllReposFromDb();
  allRepos.value = res;
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
      class="fixed z-10 flex w-full h-full overflow-auto text-black bg-slate-900 bg-opacity-60"
      @click="handleModal"
    >
      <section
        class="absolute flex flex-col items-center justify-center w-2/4 p-1 bg-white shadow-xl rounded-xl top-10 left-1/4"
      >
        <span class="flex items-center justify-center w-full h-full">
          <v-icon
            fill="black"
            name="hi-search"
            scale="1.3"
            class="ml-6 border-0"
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
            class="p-2 border border-black rounded-lg bg-zinc-200"
          >
            Ctrl
          </p>
          <p
            v-if="!searchValue"
            class="p-2 mx-2 border border-black rounded-lg bg-zinc-200"
          >
            K
          </p>
        </span>

        <li
          v-for="repo in allReposFiltered"
          :key="repo.name"
          class="flex items-center justify-center w-full h-full p-3 overflow-hidden text-xl bg-white border-b-2 border-gray-400 cursor-pointer search-results hover:bg-slate-100"
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
