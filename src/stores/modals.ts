import { defineStore } from "pinia";

const useModalsStore = defineStore("modalsStore", {
  state: () => ({
    searchModal: false,
    themeModal: false,
  }),
  actions: {
    setSearchModalOpen(state: boolean) {
      this.searchModal = state;
    },
    toggleSearchModal() {
      this.searchModal = !this.searchModal;
    },
    setThemeModalOpen(state: boolean) {
      this.themeModal = state;
    },
  },
});

export default useModalsStore;
