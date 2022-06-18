import { defineStore } from "pinia";

const useRepoStore = defineStore("repoStore", {
  state: () => ({
    repo: null as string | null,
  }),
  actions: {
    setRepo(newRepo: string) {
      this.repo = newRepo;
    },
  },
});

export default useRepoStore;
