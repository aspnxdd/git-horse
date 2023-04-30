<script setup lang="ts">
import { githubDimmedTheme, githubLightTheme } from "../../shared/constants";

const THEME_MAPPER = {
  "Github Light": githubLightTheme,
  "Github Dimmed": githubDimmedTheme,
};

const props = defineProps({
  theme: {
    type: String,
    default: "Github Light",
  },
});

const theme = THEME_MAPPER[props.theme as keyof typeof THEME_MAPPER];
</script>

<template>
  <div class="flex flex-col font-semibold">
    <div class="flex">
      <span
        :style="{
          color: theme.language,
        }"
        >{{
          `
                  function
                    `
        }}</span
      >
      <span
        :style="{
          color: theme.functionColor,
        }"
        class="ml-2"
        >{{
          `
                  isOdd 
                    `
        }}</span
      >
      <span
        :style="{
          color: theme.emphasis,
        }"
        >{{
          `
                   (
                    `
        }}</span
      >
      <span
        :style="{
          color: theme.emphasis,
        }"
        >{{
          `
                   num
                    `
        }}</span
      >
      <span
        :style="{
          color: theme.emphasis,
        }"
        >{{
          `
                   ) 
                    `
        }}</span
      >
      <span
        class="ml-2"
        :style="{
          color: theme.emphasis,
        }"
        >{{
          `
                    {
                    `
        }}</span
      >
    </div>

    <div class="ml-4">
      <span
        :style="{
          color: theme.language,
        }"
        >{{
          `
                    return
                    `
        }}</span
      >
      <span
        :style="{
          color: theme.emphasis,
        }"
        >{{
          `
                    num %
                    `
        }}</span
      >
      <span
        :style="{
          color: theme.variable,
        }"
        >{{
          `
                    1
                    `
        }}</span
      >
      <span
        :style="{
          color: theme.emphasis,
        }"
        >{{
          `
                    ===
                    `
        }}</span
      >
      <span
        :style="{
          color: theme.variable,
        }"
        >{{
          `
                    0
                    `
        }}</span
      >
    </div>
    <span
      :style="{
        color: theme.emphasis,
      }"
      >{{
        `
                }
                `
      }}</span
    >
  </div>
</template>
