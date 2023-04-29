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
  new Command("vscode", [
    `${repoStore.repo}/${repoStore.selectedFile}`,
  ]).spawn();
}
</script>

<template>
  <section
    v-if="repoStore.selectedFile"
    class="flex flex-col items-start w-full mt-2"
  >
    <div class="flex flex-row items-center gap-4 ml-2">
      <h1 class="text-lg font-bold text-text">
        File [ <i class="text-primary">{{ repoStore.selectedFile }}</i> ]
      </h1>
      <button
        class="flex items-center justify-center gap-2 text-sm text-text hover:text-text-hover"
        @click="openFileInVsCode"
      >
        <strong> View in</strong>

        <v-icon name="vi-file-type-vscode" scale="1.2" />
      </button>
    </div>

    <div
      v-if="repoDiffLines.length > 0"
      class="flex mt-4 mb-10 text-sm border-2 border-gray-500 rounded-xl bg-text-area-background"
    >
      <table class="text-left">
        <tbody class="border-r border-gray-500">
          <tr
            v-for="file in gitDiffContent.filter((diff) => diff.origin !== 'H')"
            :key="file.origin + file.diffContent + file.newLine + file.oldLine"
            :class="{
              'bg-green-addition': file.origin === '+',
              'bg-red-deletion': file.origin === '-',
            }"
          >
            <td class="w-6 h-[48px] px-2 m-1">
              {{ file.oldLine }}
            </td>
            <td class="w-6 h-[48px] px-2 m-0">
              {{ file.newLine }}
            </td>
            <td class="w-6 h-[48px] px-2 m-0">
              {{ file.origin }}
            </td>
          </tr>
        </tbody>
      </table>
      <CodeHighlighter
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

tr:first-of-type > td:first-of-type,
tr:last-of-type > td:first-of-type {
  border-top-left-radius: 0.75rem;
  border-bottom-left-radius: 0.75rem;
}

.consolas {
  font-family: "Consolas", "Courier New", Courier, monospace;
}
</style>
