import { createPinia } from "pinia";
import { createApp } from "vue";
import App from "./App.vue";
import hljs from "highlight.js/lib/core";
import javascript from "highlight.js/lib/languages/javascript";
import hljsVuePlugin from "@highlightjs/vue-plugin";
import { OhVueIcon, addIcons } from "oh-vue-icons";
import {
  GiHorseHead,
  HiFolderOpen,
  HiCloudDownload,
  HiCloudUpload,
  HiSolidChevronDoubleRight,
  HiSearch,
  HiViewGrid,
  PiHorsea,
  BiArrowUpSquare,
  BiArrowDownSquare,
  SiSpinrilla,
  ViFileTypeVscode,
  BiPalette,
} from "oh-vue-icons/icons";
import "./assets/main.postcss";

addIcons(
  GiHorseHead,
  HiFolderOpen,
  HiCloudDownload,
  HiCloudUpload,
  HiSolidChevronDoubleRight,
  HiSearch,
  HiViewGrid,
  PiHorsea,
  BiArrowUpSquare,
  BiArrowDownSquare,
  SiSpinrilla,
  ViFileTypeVscode,
  BiPalette
);

const pinia = createPinia();

hljs.registerLanguage("javascript", javascript);

createApp(App)
  .component("v-icon", OhVueIcon)
  .use(hljsVuePlugin)
  .use(pinia)
  .mount("#app");
