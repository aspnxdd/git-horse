import { createPinia } from "pinia";
import { createApp } from "vue";
import App from "./App.vue";

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
} from "oh-vue-icons/icons";

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
  ViFileTypeVscode
);

import "./assets/main.postcss";
const pinia = createPinia();

createApp(App).component("v-icon", OhVueIcon).use(pinia).mount("#app");
