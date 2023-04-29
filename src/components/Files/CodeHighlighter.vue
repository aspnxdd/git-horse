<script setup lang="ts">
const props = defineProps({
  code: {
    type: Array as PropType<
      Array<{
        text: string;
        origin: string;
      }>
    >,
    default: new Array<{
      text: string;
      origin: string;
    }>(),
  },
});

onUpdated(() => {
  const lines = document.querySelectorAll(".hljs");
  lines.forEach((element, idx) => {
    element.classList.remove("bg-green-addition");
    element.classList.remove("bg-red-deletion");
    if (props.code[idx]?.origin === "+") {
      element.classList.add("bg-green-addition");
    }
    if (props.code[idx]?.origin === "-") {
      element.classList.add("bg-red-deletion");
    }
  });
});
</script>

<template>
  <div class="flex flex-col bg-transparent">
    <highlightjs
      v-for="(line, idx) in props.code"
      :key="line.origin + line.text + idx"
      :code="line.text"
      class="h-[48px]"
    />
  </div>
</template>

<style scoped>
:deep(.hljs) {
  height: 48px;
  overflow-x: auto;
  overflow-y: hidden;
}

:deep(span) {
  font-family: "Consolas", "Courier New", Courier, monospace !important;
}
</style>
