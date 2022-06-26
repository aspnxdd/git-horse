<script setup lang="ts">
import { Replace, FileStatus, GitStatus, TreeItem } from "@types";
import { PropType } from "vue";
import TreeRow from "./TreeRow.vue";

const props = defineProps({
  filesModifiedNames: {
    type: Array as PropType<Replace<FileStatus, "status", GitStatus[]>[]>,
    default: new Array<Replace<FileStatus, "status", GitStatus>>(),
  },
});
const tree = ref<TreeItem>();
const splitPath = (p: string) => p.split("/").filter(Boolean);

const createTree = (items: string[][]) => {
  const _tree: TreeItem = { isFile: false, depth: 0 };
  items.forEach((parts) => {
    let prevTree = _tree;
    parts.forEach((part, index, total) => {
      const isFile = index === total.length - 1;
      if (!prevTree[part as string])
        prevTree[part as string] = { isFile, depth: index };
      prevTree = prevTree[part as string] as TreeItem;
    });
  });
  console.log("tree", _tree);
  return _tree;
};
tree.value = createTree(
  props.filesModifiedNames.map((e) => splitPath(e.fileName))
);
</script>

<template>
  <ul v-for="(val, key) in Object.entries(tree!).slice(2)" :key="key">
    <TreeRow :depth="val[1].depth" :name="val[0]" :rest="(val[1] as TreeItem)" />
  </ul>
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
