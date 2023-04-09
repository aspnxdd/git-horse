import { defineStore } from "pinia";

const useModalsStore = defineStore("modalsStore", {
  state: () => ({
    searchModal: false,
  }),
  actions: {
    setSearchModalOpen(state: boolean) {
      this.searchModal = state;
    },
    toggleSearchModal() {
      this.searchModal = !this.searchModal;
    },
  },
});

export default useModalsStore;
