<script setup lang="ts">
import { PropType } from "vue";
import { GitDiff, GitStatus } from "@types";

const props = defineProps({
  repoDiffLines: {
    type: Array as PropType<GitDiff[]>,
    default: new Array<GitDiff>(),
  },
  filesModifiedNames: {
    type: Array as PropType<{ fileName: string; status: GitStatus }[]>,
    default: null,
  },
  selectedFile: {
    type: String as PropType<string | null>,
    default: null,
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
      if (diff.origin == "F") return;
      const nextBreakline =
        breaklines[index + 1]?.breakline ?? props.repoDiffLines.length;
      if (i > breakline && i <= nextBreakline) {
        if (filtered[fileName as string]?.length > 0) {
          filtered[fileName as string].push(diff);
        } else {
          filtered[fileName as string] = [diff];
        }
      }
    });
  }
  console.log("filtered", filtered);
  repoDiffLinesFiltered.value = filtered;
}

watch(props, () => {
  filterFileDiff();
  displayFileDiff();
});

const displayFileDiff = () => {
  console.log("selectedFile", props.selectedFile);

  gitDiffContent.value =
    repoDiffLinesFiltered.value[props.selectedFile as string];
};
</script>

<template>
  <section
    v-if="repoDiffLines.length > 0 && filesModifiedNames.length > 0"
    class="flex flex-col items-start mt-2"
  >
    <h1 class="font-bold text-lg">Changed file ({{ props.selectedFile }})</h1>
    <code
      v-if="repoDiffLines.length > 0"
      class="list-none p-2 bg-[#4c4653] rounded-xl m-2 text-xs overflow-scroll h-[50vh] break-words w-[90%] mb-10"
    >
      <table class="table-auto w-full text-left">
        <tbody>
          <tr
            v-for="(file, index) in gitDiffContent"
            :key="index"
            :class="{
              'bg-green-800': file.origin == '+',
              'bg-red-700': file.origin == '-',
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
