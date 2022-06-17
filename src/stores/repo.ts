import { defineStore } from "pinia";

const useLogsStore = defineStore("repoStore", {
  state: () => ({
    repo: null as string|null,
  }),
  actions: {
    setLogs(newLog: string) {
      this.logs.push(newLog);
    },
    clearLogs() {
      this.logs = [];
    },
  },
});

export default useLogsStore;