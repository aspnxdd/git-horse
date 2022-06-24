<script setup lang="ts">
import { PropType } from "vue";
import { Repos } from "@types";
import { invoke } from "@tauri-apps/api/tauri";
import { useRepoStore } from "@stores";

const allRepos = ref<Repos[]>([]);
const allReposFiltered = ref<Repos[]>([]);

const repoStore = useRepoStore();

function filterRepos(query: string) {
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

interface Emits {
  (e: "close:modal", open: boolean): void;
}
const emits = defineEmits<Emits>();

function handleModal(e: MouseEvent) {
  if ((e.target as HTMLDivElement).nodeName === "DIV") closeModal();
}

function closeModal() {
  emits("close:modal", false);
}

function selectRepo(path: string) {
  repoStore.setRepo(path);
  closeModal();
}
onUpdated(async () => {
 const res = await invoke<Repos[]>("db_get_all");
   allRepos.value = res.filter((repo)=> repo.name !=="last_opened_repo")
  console.log("allRepos:", allRepos.value);
});
</script>

<template>
  <Transition name="fade">
    <div
      v-if="modalOpen"
      class="flex w-full h-full fixed overflow-auto bg-slate-900 bg-opacity-60 text-black"
      @click="handleModal"
    >
      <div class="flex flex-col justify-center items-center w-full h-full">
        <section
          class="bg-white absolute overflow-hidden flex flex-col items-center w-2/4 rounded-xl h-max m-auto shadow-xl top-16 left-1/3"
        >
          <span class="flex justify-center items-center w-full h-full">
            <v-icon
              fill="black"
              name="hi-search"
              scale="1.3"
              class="border-0 ml-6"
            />

            <input
              autofocus
              id="search"
              placeholder="Type * to show all repos..."
              class="w-full h-full p-5 text-xl outline-white"
              @input="(e)=>filterRepos((e.target as HTMLInputElement).value)"
            />
          </span>

          <!-- <TransitionGroup name="list"> -->
          <li
            v-for="repo in allReposFiltered"
            @click="() => selectRepo(repo.path)"
            :key="repo.name"
            class="text-xl border-b-2 border-gray-400 p-2 w-full h-full flex justify-center items-center bg-white sticky overflow-hidden search-results"
          >
            {{ repo.name }}
          </li>
          <!-- </TransitionGroup> -->
        </section>
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

.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
}

.hamb {
  animation: slide-right 300ms ease-in-out forwards;
  animation-delay: 200ms;
  animation-iteration-count: 1;
}
.line {
  transition: transform 300ms ease-in-out;
}
.line:nth-child(1) {
  background-color: #800080;
  display: block;
  width: 40px;
  height: 3px;
  transform: rotate(45deg) translateY(10px) translateX(-4px);

  margin-block: 10px;
}
.line:nth-child(2) {
  background-color: #800080;
  display: block;
  width: 40px;
  height: 3px;
  margin-block: 10px;
  transform: rotate(-45deg) translateY(-13px);
}
.hamb:hover .line:nth-child(2) {
  transform: rotate(0deg) translateY(-13px) translateX(-10px) scaleX(0.8);
}
.hamb:hover .line:nth-child(1) {
  transform: rotate(0deg) translateY(-0px) translateX(-10px) scaleX(0.8);
}
</style>
