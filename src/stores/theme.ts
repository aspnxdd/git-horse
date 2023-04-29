import { defineStore } from "pinia";
const DEFAULT_THEME = "github-dimmed";

const useThemeStore = defineStore("themeStore", {
  state: () => ({
    theme: DEFAULT_THEME,
  }),
  actions: {
    setTheme(newTheme: string) {
      this.theme = newTheme;
    },
  },
});

export default useThemeStore;
