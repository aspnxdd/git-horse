<script setup lang="ts">
import type { PropType } from "vue";

import { GitStatus, GitStatusColors } from "src/shared/constants";

interface Emits {
  (e: "update:checked", checked: boolean): void;
  (e: "display"): void;
}

const emits = defineEmits<Emits>();

function updateChecked(checked: boolean) {
  emits("update:checked", checked);
}

function displayFile() {
  emits("display");
}

const props = defineProps({
  fileName: {
    type: String,
    default: null,
  },
  status: {
    type: String as PropType<keyof typeof GitStatus>,
    default: null,
  },
  checked: {
    type: Boolean,
    default: true,
  },
});

function getStatusColor(status: string) {
  if (Object.hasOwn(GitStatusColors, status)) {
    return GitStatusColors[status as keyof typeof GitStatusColors];
  }
  return "text-[#f546fa]";
}

const colorStatus = getStatusColor(props.status);
</script>

<template>
  <div class="relative flex items-center">
    <span :class="`${colorStatus} left-0 mr-2 font-bold`"
      >[{{ status.charAt(0) }}]</span
    >
    <input
      :id="fileName"
      :ref="fileName"
      type="checkbox"
      class="accent-pink-500 mr-2"
      :checked="props.checked"
      @input="(event)=>updateChecked((event.target as HTMLInputElement).checked)"
    />
    <p :for="fileName" class="hover:text-slate-400" @click="displayFile">
      {{ fileName }}
    </p>
  </div>
</template>

<style scoped></style>
