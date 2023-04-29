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
    if (idx === 0) {
      element.classList.add("rounded-tr-xl");
    }
    if (idx === lines.length - 1) {
      element.classList.add("rounded-br-xl");
    }
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
  <div class="flex flex-col">
    <highlightjs
      v-for="(line, idx) in props.code"
      :key="line.origin + line.text + idx"
      :code="line.text"
    />
  </div>
</template>

<style scoped>
:deep(.hljs) {
  height: 48px;
  width: 65vw;
  overflow-x: auto;
  overflow-y: hidden;
}

:deep(span) {
  font-family: "Consolas", "Courier New", Courier, monospace !important;
}
</style>
