<script setup lang="ts">
import { PropType } from "vue";
import { GitDiff } from "@types";

defineProps({
  repoDiffLines: {
    type: Array as PropType<GitDiff[]>,
    default: new Array<GitDiff>(),
  },
});
</script>

<template>
  <section class="flex flex-col items-start">
    <h1 class="font-bold text-lg">Changed files:</h1>
    <code
      v-if="repoDiffLines.length > 0"
      class="list-none p-2 bg-[#21325a] rounded-xl m-2 text-xs"
    >
      <table class="table-auto w-full text-left">
        <tbody>
          <tr
            v-for="(file, index) in repoDiffLines"
            :key="index"
            :class="{
              'bg-green-500': file.origin == '+',
              'bg-red-500': file.origin == '-',
            }"
          >
            <td>{{ file.oldLine }}</td>
            <td>{{ file.newLine }}</td>
            <td>{{ file.origin }}</td>
            <td>{{ file.diffContent }}</td>
          </tr>
        </tbody>
      </table>
    </code>
  </section>
</template>

<style scoped>
main {
  background: rgb(55, 55, 149);
  background: linear-gradient(
    120deg,
    rgba(55, 55, 149, 1) 0%,
    rgba(69, 123, 229, 1) 100%
  );
  cursor: default;
}
</style>
