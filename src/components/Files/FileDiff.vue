<script setup lang="ts">
import type { PropType } from "vue";
import type { GitDiff } from "src/shared/types";

import { GitStatus } from "src/shared/constants";
import { useRepoStore } from "@stores";

const props = defineProps({
  repoDiffLines: {
    type: Array as PropType<GitDiff[]>,
    default: new Array<GitDiff>(),
  },
  filesModifiedNames: {
    type: Array as PropType<
      { fileName: string; status?: keyof typeof GitStatus }[]
    >,
    default: null,
  },
});

const repoStore = useRepoStore();

const gitDiffContent = ref<GitDiff[]>([]);
const repoDiffLinesFiltered = ref<Record<string, GitDiff[]>>({});

function _getOriginAndCurrentFileName(diffContent: string) {
  const originName = diffContent.split(" ")[2].split("/").slice(1).join();
  const currentName = diffContent
    .split(" ")[3]
    .split("/")
    .slice(1)
    .join("/")
    .split("\n")[0];
  return [originName, currentName];
}

function filterFileDiff() {
  const filtered: Record<string, GitDiff[]> = {};
  const breaklines: {
    fileName: string;
    fileStartsAtLine: number;
  }[] = [];
  for (let idx = 0; idx < props.repoDiffLines.length; idx++) {
    const diff = props.repoDiffLines[idx];
    const fileName = props.filesModifiedNames.find(({ fileName }) => {
      if (diff.origin !== "F") {
        return false;
      }
      const [originName, currentName] = _getOriginAndCurrentFileName(
        diff.diffContent
      );
      return originName === fileName || currentName === fileName;
    })?.fileName;
    if (!fileName) {
      continue;
    }
    breaklines.push({ fileName, fileStartsAtLine: idx });
  }
  for (let idx = 0; idx < breaklines.length; idx++) {
    const { fileName, fileStartsAtLine } = breaklines[idx];
    props.repoDiffLines.forEach((diff, i) => {
      if (diff.origin === "F") {
        return;
      }
      const nextBreakline =
        breaklines[idx + 1]?.fileStartsAtLine ?? props.repoDiffLines.length;
      if (i > fileStartsAtLine && i <= nextBreakline) {
        if (filtered[fileName]?.length > 0) {
          filtered[fileName].push(diff);
        } else {
          filtered[fileName] = [diff];
        }
      }
    });
  }
  repoDiffLinesFiltered.value = filtered;
}

watch(props, () => {
  filterFileDiff();
  displayFileDiff();
});

const displayFileDiff = () => {
  if (!repoStore.selectedFile) return;
  gitDiffContent.value = repoDiffLinesFiltered.value[repoStore.selectedFile];
};
</script>

<template>
  <section
    v-if="repoStore.selectedFile"
    class="flex flex-col items-start mt-2"
  >
    <h1 class="font-bold text-lg">
      File [<i class="text-[#cfcf44]">{{ repoStore.selectedFile }}</i
      >]
    </h1>
    <code
      v-if="repoDiffLines.length > 0"
      class="list-none p-2 bg-[#4c4653] rounded-xl m-2 text-sm overflow-auto h-[50vh] break-words w-[90%] mb-10"
    >
      <table class="table-auto w-full text-left">
        <tbody>
          <tr
            v-for="file in gitDiffContent"
            :key="file.origin + file.diffContent + file.newLine + file.oldLine"
            :class="{
              'bg-green-800': file.origin === '+',
              'bg-red-700': file.origin === '-',
            }"
          >
            <td class="w-6">{{ file.oldLine }}</td>
            <td class="w-6">{{ file.newLine }}</td>
            <td class="w-6">{{ file.origin }}</td>
            <td class="consolas">{{ file.diffContent }}</td>
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

.consolas {
  font-family: "Consolas", "Courier New", Courier, monospace;
}
</style>
