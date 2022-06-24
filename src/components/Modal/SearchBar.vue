<script setup lang="ts">
import { PropType } from "vue";
import { Repos } from "@types";
import { invoke } from "@tauri-apps/api/tauri";
import { useRepoStore } from "@stores";
import { debounce } from "@utils";

defineProps({
  modalOpen: {
    type: Boolean,
    default: false,
  },
  allRepos: {
    type: Array as PropType<Repos[]>,
    default: null,
  },
});

const allRepos = ref<Repos[]>([]);
const allReposFiltered = ref<Repos[]>([]);

const repoStore = useRepoStore();

function queryFn(query: string) {
  if (query == "") {
    allReposFiltered.value = [];
    return;
  }
  if (query == "*") {
    allReposFiltered.value = allRepos.value;
    return;
  }
  allReposFiltered.value = allRepos.value.filter((repo) => {
    return repo.name.toLowerCase().includes(query.toLowerCase());
  });
  return;
}
const filterReposDebounced = debounce(queryFn, 300);

function filterReposHandler(query: string) {
  query == "" ? queryFn(query) : filterReposDebounced(query);
}

interface Emits {
  (e: "close:modal", open: boolean): void;
}
const emits = defineEmits<Emits>();

function handleModal(e: MouseEvent) {
  if ((e.target as HTMLDivElement).nodeName === "DIV") {
    closeModal();
  }
}

function closeModal() {
  allReposFiltered.value = [];
  emits("close:modal", false);
}

function selectRepo(path: string) {
  repoStore.setRepo(path);
  closeModal();
}
onUpdated(async () => {
  const res = await invoke<Repos[]>("db_get_all");
  allRepos.value = res.filter((repo) => repo.name !== "last_opened_repo");
  console.log("allRepos:", allRepos.value);
});
</script>

<template>
  <Transition name="fade">
    <div
      v-if="modalOpen"
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
            autofocus
            placeholder="Type * to show all repos..."
            class="w-full h-full p-5 text-xl outline-white"
            @input="(e)=>filterReposHandler((e.target as HTMLInputElement).value)"
          />
        </span>

        <!-- <TransitionGroup name="list"> -->
        <li
          v-for="repo in allReposFiltered"
          :key="repo.name"
          class="text-xl border-b-2 border-gray-400 p-3 w-full h-full flex justify-center items-center bg-white overflow-hidden search-results cursor-pointer hover:bg-slate-100"
          @click="() => selectRepo(repo.path)"
        >
          {{ repo.name }}
        </li>
        <!-- </TransitionGroup> -->
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
</style>
