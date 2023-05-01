<!-- eslint-disable @typescript-eslint/no-non-null-assertion -->
<script setup lang="ts">
import { SideBar, FilesView } from "@components/Layout";
import { useRepoStore, useThemeStore } from "@stores";
import { SearchBarModal, ThemeSelectorModal } from "@components/Modal";
import {
  githubDimmedTheme,
  githubLightTheme,
  type Theme,
} from "./shared/constants";
import {
  getLastOpenedRepoFromDb,
  getThemeFromDb,
  setThemeToDb,
} from "src/adapter/db";
import { Splitpanes, Pane } from "splitpanes";
import "splitpanes/dist/splitpanes.css";

const DEFAULT_THEME = githubDimmedTheme;

const repoStore = useRepoStore();
const themeStore = useThemeStore();

onMounted(async () => {
  const theme = await getThemeFromDb().catch(() => "github-dimmed");
  themeStore.setTheme(theme);
  const html = document.querySelector("html")!;
  html.attributes.setNamedItem(document.createAttribute("data-theme"));
  html.attributes.getNamedItem("data-theme")!.value = theme;
  try {
    const res = await getLastOpenedRepoFromDb();
    repoStore.setRepo(res);
  } catch (_) {}
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

const bg = ref(DEFAULT_THEME.bg);
const language = ref(DEFAULT_THEME.language);
const functionColor = ref(DEFAULT_THEME.functionColor);
const variable = ref(DEFAULT_THEME.variable);
const meta = ref(DEFAULT_THEME.meta);
const symbol = ref(DEFAULT_THEME.symbol);
const formula = ref(DEFAULT_THEME.formula);
const selector = ref(DEFAULT_THEME.selector);
const subst = ref(DEFAULT_THEME.subst);
const section = ref(DEFAULT_THEME.section);
const bullet = ref(DEFAULT_THEME.bullet);
const emphasis = ref(DEFAULT_THEME.emphasis);
const strong = ref(DEFAULT_THEME.strong);
const addition = ref(DEFAULT_THEME.addition);
const deletion = ref(DEFAULT_THEME.deletion);
const additionBg = ref(DEFAULT_THEME.additionBg);
const deletionBg = ref(DEFAULT_THEME.deletionBg);

watch(
  () => themeStore.theme,
  async (theme) => {
    const html = document.querySelector("html")!;
    html.attributes.setNamedItem(document.createAttribute("data-theme"));
    html.attributes.getNamedItem("data-theme")!.value = theme;
    if (theme === "github-light") {
      themeSetter(githubLightTheme);
    } else if (theme === "github-dimmed") {
      themeSetter(githubDimmedTheme);
    }
    await setThemeToDb(theme);
  }
);
</script>

<template>
  <search-bar-modal />
  <theme-selector-modal />
  <splitpanes>
    <pane size="20" max-size="30" min-size="20">
      <side-bar />
    </pane>
    <pane size="80">
      <files-view />
    </pane>
  </splitpanes>
</template>

<style>
::-webkit-scrollbar {
  width: 10px;
  height: 10px;
}

/* Track */
::-webkit-scrollbar-track {
  @apply bg-text-area-background;
}

/* Handle */
::-webkit-scrollbar-thumb {
  background: #888;
}

/* Handle on hover */
::-webkit-scrollbar-thumb:hover {
  background: #787878;
}

::-webkit-scrollbar-corner {
  background: rgba(0, 0, 0, 0);
}

.splitpanes__pane {
  font-family: Helvetica, Arial, sans-serif;
  color: rgba(255, 255, 255, 0.6);
  overflow: auto;
  @apply bg-background;
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
