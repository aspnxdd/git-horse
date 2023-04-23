<script setup lang="ts">
import type { PropType } from "vue";
import type { GitDiff } from "src/shared/types";

import { Command } from "@tauri-apps/api/shell";
import { GitStatus } from "src/shared/constants";
import { useRepoStore } from "@stores";
import { CodeHighlighter } from ".";

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
  console.log({ filtered });
  repoDiffLinesFiltered.value = filtered;
}

watch(props, () => {
  filterFileDiff();
  displayFileDiff();
});

const displayFileDiff = () => {
  if (!repoStore.selectedFile) return;
  gitDiffContent.value = repoDiffLinesFiltered.value[
    repoStore.selectedFile
  ].reduce((acc, element) => {
    // add extra empty line to the diff content
    acc.push({
      ...element,
      diffContent: element.diffContent.replaceAll("\n", ""),
    });
    return acc;
  }, [] as GitDiff[]);
};

function openFileInVsCode() {
  if (!repoStore.selectedFile || !repoStore.repo) return;
  console.log({ file: repoStore.selectedFile, repo: repoStore.repo });
  new Command("vscode", [
    `${repoStore.repo}/${repoStore.selectedFile}`,
  ]).spawn();
}
</script>

<template>
  <section
    v-if="repoStore.selectedFile"
    class="flex flex-col items-start mt-2 w-[80vw]"
  >
    <div class="flex flex-row items-center gap-4 ml-2">
      <h1 class="font-bold text-lg">
        File [ <i class="text-primary">{{ repoStore.selectedFile }}</i> ]
      </h1>
      <button
        class="text-sm flex justify-center items-center gap-2 hover:text-slate-300"
        @click="openFileInVsCode"
      >
        <strong> View in</strong>

        <v-icon name="vi-file-type-vscode" scale="1.2" />
      </button>
    </div>

    <div
      v-if="repoDiffLines.length > 0"
      class="bg-[#4c4653] rounded-xl text-sm w-full mb-10 mt-4 flex overflow-hidden border border-gray-500"
    >
      <table class="w-[10%] text-left">
        <tbody class="w-full border-r border-gray-500">
          <tr
            v-for="file in gitDiffContent.filter((diff) => diff.origin !== 'H')"
            :key="file.origin + file.diffContent + file.newLine + file.oldLine"
            class="h-[48px] p-0 m-0"
            :class="{
              'bg-green-800': file.origin === '+',
              'bg-red-700': file.origin === '-',
            }"
          >
            <td class="w-6 h-[48px] px-2 m-0">{{ file.oldLine }}</td>
            <td class="w-6 h-[48px] px-2 m-0">{{ file.newLine }}</td>
            <td class="w-6 h-[48px] px-2 m-0">{{ file.origin }}</td>
          </tr>
        </tbody>
      </table>
      <CodeHighlighter
        class="w-[90%]"
        :code="
          (gitDiffContent ?? [])
            .filter((diff) => diff.origin !== 'H')
            .map((diff) => {
              return {
                text: diff.diffContent,
                origin: diff.origin,
              };
            })
        "
      />
    </div>
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
