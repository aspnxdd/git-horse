<!-- eslint-disable @typescript-eslint/no-non-null-assertion -->
<script setup lang="ts">
import { SideBar, FilesView } from "@components/Layout";
import { invoke } from "@tauri-apps/api/tauri";
import { useRepoStore, useThemeStore } from "@stores";
import { SearchBar, ThemeSelector } from "@components/Modal";
import {
  githubDimmedTheme,
  githubLightTheme,
  type Theme,
} from "./shared/constants";
import { Splitpanes, Pane } from "splitpanes";
import "splitpanes/dist/splitpanes.css";

const repoStore = useRepoStore();
const themeStore = useThemeStore();

const defaultTheme = githubDimmedTheme;

onMounted(async () => {
  const theme = await invoke<string>("read_theme");
  themeStore.setTheme(theme);
  const html = document.querySelector("html")!;
  html.attributes.setNamedItem(document.createAttribute("data-theme"));
  html.attributes.getNamedItem("data-theme")!.value = theme;
  const res = await invoke<string>("read_last_opened_repo");
  repoStore.setRepo(res);
});

function themeSetter(newTheme: Theme) {
  bg.value = newTheme.bg;
  language.value = newTheme.language;
  functionColor.value = newTheme.functionColor;
  variable.value = newTheme.variable;
  meta.value = newTheme.meta;
  symbol.value = newTheme.symbol;
  formula.value = newTheme.formula;
  selector.value = newTheme.selector;
  subst.value = newTheme.subst;
  section.value = newTheme.section;
  bullet.value = newTheme.bullet;
  emphasis.value = newTheme.emphasis;
  strong.value = newTheme.strong;
  addition.value = newTheme.addition;
  deletion.value = newTheme.deletion;
  additionBg.value = newTheme.additionBg;
  deletionBg.value = newTheme.deletionBg;
}

const bg = ref(defaultTheme.bg);
const language = ref(defaultTheme.language);
const functionColor = ref(defaultTheme.functionColor);
const variable = ref(defaultTheme.variable);
const meta = ref(defaultTheme.meta);
const symbol = ref(defaultTheme.symbol);
const formula = ref(defaultTheme.formula);
const selector = ref(defaultTheme.selector);
const subst = ref(defaultTheme.subst);
const section = ref(defaultTheme.section);
const bullet = ref(defaultTheme.bullet);
const emphasis = ref(defaultTheme.emphasis);
const strong = ref(defaultTheme.strong);
const addition = ref(defaultTheme.addition);
const deletion = ref(defaultTheme.deletion);
const additionBg = ref(defaultTheme.additionBg);
const deletionBg = ref(defaultTheme.deletionBg);

watch(
  () => themeStore.theme,
  async (theme) => {
    const html = document.querySelector("html")!;
    html.attributes.getNamedItem("data-theme")!.value = theme;
    if (theme === "github-light") {
      themeSetter(githubLightTheme);
    } else if (theme === "github-dimmed") {
      themeSetter(githubDimmedTheme);
    }
    await invoke("write_theme", { key: theme });
  }
);
</script>

<template>
  <SearchBar />
  <ThemeSelector />
  <splitpanes>
    <pane size="20" max-size="30" min-size="20">
      <SideBar />
    </pane>
    <pane size="80">
      <FilesView />
    </pane>
  </splitpanes>
</template>

<style>
.splitpanes__pane {
  font-family: Helvetica, Arial, sans-serif;
  color: rgba(255, 255, 255, 0.6);
  overflow: auto;
  background: v-bind(bg);
}
.splitpanes--vertical > .splitpanes__splitter {
  min-width: 1px;
  cursor: col-resize;
  width: 3px;
}
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

pre code.hljs {
  display: block;
  overflow-x: auto;
  padding: 1em;
}
code.hljs {
  padding: 3px 5px;
}

.hljs {
  color: #adbac7;
  background: v-bind(bg);
}

.hljs-doctag,
.hljs-keyword,
.hljs-meta .hljs-keyword,
.hljs-template-tag,
.hljs-template-variable,
.hljs-type,
.hljs-variable.language_ {
  color: v-bind(language);
}
.hljs-title,
.hljs-title.class_,
.hljs-title.class_.inherited__,
.hljs-title.function_ {
  color: v-bind(functionColor);
}
.hljs-attr,
.hljs-attribute,
.hljs-literal,
.hljs-meta,
.hljs-number,
.hljs-operator,
.hljs-selector-attr,
.hljs-selector-class,
.hljs-selector-id,
.hljs-variable {
  color: v-bind(variable);
}
.hljs-meta .hljs-string,
.hljs-regexp,
.hljs-string {
  color: v-bind(meta);
}
.hljs-built_in,
.hljs-symbol {
  color: v-bind(symbol);
}
.hljs-code,
.hljs-comment,
.hljs-formula {
  color: v-bind(formula);
}
.hljs-name,
.hljs-quote,
.hljs-selector-pseudo,
.hljs-selector-tag {
  color: v-bind(selector);
}
.hljs-subst {
  color: v-bind(subst);
}
.hljs-section {
  color: v-bind(section);
  font-weight: 700;
}
.hljs-bullet {
  color: v-bind(bullet);
}
.hljs-emphasis {
  color: v-bind(emphasis);
  font-style: italic;
}
.hljs-strong {
  color: v-bind(strong);
  font-weight: 700;
}
.hljs-addition {
  color: v-bind(addition);
  background-color: v-bind(additionBg);
}
.hljs-deletion {
  color: v-bind(deletion);
  background-color: v-bind(deletionBg);
}
</style>
