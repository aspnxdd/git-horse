<!-- eslint-disable @typescript-eslint/no-non-null-assertion -->
<script setup lang="ts">
import { SideBar, FilesView } from "@components/Layout";
import { invoke } from "@tauri-apps/api/tauri";
import { useRepoStore, useThemeStore } from "@stores";
import { SearchBar, ThemeSelector } from "@components/Modal";

const repoStore = useRepoStore();
const themeStore = useThemeStore();

onMounted(async () => {
  const theme = await invoke<string>("read_theme");
  themeStore.setTheme(theme);
  const html = document.querySelector("html")!;
  html.attributes.setNamedItem(document.createAttribute("data-theme"));
  html.attributes.getNamedItem("data-theme")!.value = theme;
  const res = await invoke<string>("read_last_opened_repo");
  repoStore.setRepo(res);
});

const bg = ref("#24292e");
const language = ref("#f47067");
const functionColor = ref("#dcbdfb");
const variable = ref("#6cb6ff");
const meta = ref("#96d0ff");
const symbol = ref("#f69d50");
const formula = ref("#768390");
const selector = ref("#8ddb8c");
const subst = ref("#adbac7");
const section = ref("#316dca");
const bullet = ref("#eac55f");
const emphasis = ref("#adbac7");
const strong = ref("#adbac7");
const addition = ref("#b4f1b4");
const deletion = ref("#ffd8d3");
const additionBg = ref("#f0fff4");
const deletionBg = ref("#ffeef0");

watch(
  () => themeStore.theme,
  async (theme) => {
    const html = document.querySelector("html")!;
    html.attributes.getNamedItem("data-theme")!.value = theme;
    if (theme === "github-light") {
      bg.value = "#fff";
      language.value = "#b8868b";
      functionColor.value = "#6f42c1";
      variable.value = "#005cc5";
      meta.value = "#032f62";
      symbol.value = "#e36209";
      formula.value = "#6a737d";
      selector.value = "#22863a";
      subst.value = "#24292e";
      section.value = "#005cc5";
      bullet.value = "#735c0f";
      emphasis.value = "#24292e";
      strong.value = "#24292e";
      addition.value = "#22863a";
      deletion.value = "#b31d28";
      additionBg.value = "#f0fff4";
      deletionBg.value = "#ffeef0";
    } else if (theme === "github-dimmed") {
      bg.value = "#24292e";
      language.value = "#f47067";
      functionColor.value = "#dcbdfb";
      variable.value = "#6cb6ff";
      meta.value = "#96d0ff";
      symbol.value = "#f69d50";
      formula.value = "#768390";
      selector.value = "#8ddb8c";
      subst.value = "#adbac7";
      section.value = "#316dca";
      bullet.value = "#eac55f";
      emphasis.value = "#adbac7";
      strong.value = "#adbac7";
      addition.value = "#b4f1b4";
      deletion.value = "#ffd8d3";
      additionBg.value = "#f0fff4";
      deletionBg.value = "#ffeef0";
    }
    await invoke("write_theme", { key: theme });
  }
);
</script>

<template>
  <SearchBar />
  <ThemeSelector />
  <SideBar />
  <FilesView />
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  display: flex;
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
