<script setup lang="ts">
import { PropType } from "vue";
import { GitDiff, GitStatus, Replace, FileStatus } from "@types";
import TreeC from "./TreeC.vue";
const props = defineProps({
  repoDiffLines: {
    type: Array as PropType<GitDiff[]>,
    default: new Array<GitDiff>(),
  },
  filesModifiedNames: {
    type: Array as PropType<Replace<FileStatus, "status", GitStatus[]>[]>,
    default: new Array<Replace<FileStatus, "status", GitStatus>>(),
  },
});
const gitDiffContent = ref<GitDiff[]>([]);
const repoDiffLinesFiltered = ref<Record<string, GitDiff[]>>({});

function filterFileDiff() {
  const filtered: Record<string, GitDiff[]> = {};
  const breaklines: {
    fileName: string;
    breakline: number;
  }[] = [];
  for (const [index, diff] of props.repoDiffLines.entries()) {
    const fileName = props.filesModifiedNames.filter((e) => {
      return diff.diffContent.includes(e.fileName) && diff.origin == "F";
    })[0]?.fileName;
    if (!fileName) continue;
    breaklines.push({ fileName, breakline: index });
  }

  for (const [index, { fileName, breakline }] of breaklines.entries()) {
    props.repoDiffLines.forEach((diff, i) => {
      const nextBreakline =
        breaklines[index + 1]?.breakline ?? props.repoDiffLines.length;
      if (i > breakline && i <= nextBreakline) {
        if (
          filtered[fileName as string] &&
          filtered[fileName as string].length > 0
        ) {
          filtered[fileName as string].push(diff);
        } else {
          filtered[fileName as string] = [diff];
        }
      }
    });
  }
  repoDiffLinesFiltered.value = filtered;
}


onUpdated(() => {
  filterFileDiff();
  if (gitDiffContent.value.length == 0)
    gitDiffContent.value =
      repoDiffLinesFiltered.value[props.filesModifiedNames[0].fileName];
});
function displayFileDiff(fileName: string) {
  gitDiffContent.value = repoDiffLinesFiltered.value[fileName as string];
}
</script>

<template>
  <nav
    class="absolute right-4 top-4 rounded-xl w-[15rem] flex flex-col text-white cursor-default overflow-hidden p-4"
  >
    <TreeC :files-modified-names="filesModifiedNames" />
  </nav>
  <section class="flex flex-col items-start">
    <h1 class="font-bold text-lg">Changed files:</h1>
    <code
      v-if="repoDiffLines.length > 0"
      class="list-none p-2 bg-[#21325a] rounded-xl m-2 text-xs overflow-scroll max-w-[65vw] max-h-[60vh] break-words"
    >
      <table class="table-auto w-full text-left">
        <tbody>
          <tr
            v-for="(file, index) in gitDiffContent"
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
