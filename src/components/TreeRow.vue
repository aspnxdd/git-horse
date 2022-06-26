<script setup lang="ts">
import { TreeItem } from "@types";
import { PropType } from "vue";

const rest = ref<string[]>();
const props = defineProps({
  name: {
    type: String,
    default: null,
  },
  depth: {
    type: Number,
    default: null,
  },
  rest: {
    type: Object as PropType<TreeItem>,
    default: null,
  },
});
console.log("props rest:", props.rest);
rest.value = Object.keys(props.rest).filter(
  (e) => e != "isFile" && e != "depth"
);
console.log("rest value:", rest.value);
</script>

<template>
  <div class="text-left">
    <p
      :class="{
        'pl-0': depth === 0,
        'pl-[1rem]': depth === 1,
        'pl-[2rem]': depth === 2,
        'pl-[3rem]': depth === 3,
        'pl-[4rem]': depth === 4,
        'pl-[5rem]': depth === 5,
        'pl-[6rem]': depth === 6,
        'pl-[7rem]': depth === 7,
        'pl-[8rem]': depth === 8,
        'pl-[9rem]': depth === 9,
      }"
    >
      {{ name }}
    </p>
    <ul v-for="(val, index) in rest" :key="index">
      <TreeRow
        :depth="props.rest[val].depth"
        :name="rest[index]"
        :rest="props.rest[val]"
      />
    </ul>
  </div>
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

nav {
  background: rgb(8, 8, 111);
  background: linear-gradient(
    45deg,
    rgba(8, 8, 111, 1) 0%,
    rgb(38, 172, 20) 100%
  );
  height: calc(100vh - 2rem);
}
</style>
