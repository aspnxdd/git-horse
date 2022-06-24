<script setup lang="ts">
import { SideBar, FilesView } from "@components/Layout";
</script>

<template>
  <div class="flex">
    <SideBar />
    <FilesView />
  </div>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
</style>
