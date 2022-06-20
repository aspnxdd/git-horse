import { defineStore } from "pinia";

const useRepoStore = defineStore("repoStore", {
  state: () => ({
    repo: null as string | null,
    activeBranch: null as string | null,
  }),
  actions: {
    setRepo(newRepo: string) {
      this.repo = newRepo;
    },
    setActiveBranch(newBranch: string) {
      this.activeBranch = newBranch;
    },
  },
});

export default useRepoStore;
