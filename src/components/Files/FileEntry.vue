<script setup lang="ts">
import type { PropType } from "vue";

import { GitStatus } from "src/shared/constants";

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
  isInput: {
    type: Boolean,
    default: true,
  },
});

function getStatusColor() {
  if (props.status === "Modified") {
    return "border-gitstatus-modified text-gitstatus-modified bg-gitstatus-modified/20";
  }
  if (props.status === "New") {
    return "border-gitstatus-new text-gitstatus-new bg-gitstatus-new/20";
  }
  if (props.status === "Deleted") {
    return "border-gitstatus-deleted text-gitstatus-deleted bg-gitstatus-deleted/20";
  }
  return "border-gitstatus-unknown text-gitstatus-unknown bg-gitstatus-unknown/20";
}
</script>

<template>
  <div class="relative flex items-center">
    <span
      class="border-2 px-[7px] rounded-xl left-0 mr-2 font-bold"
      :class="getStatusColor()"
      >{{ status.charAt(0) }}</span
    >

    <label
      :for="fileName"
      class="flex justify-center mr-2 break hover:text-text-hover"
    >
      <input
        v-if="isInput"
        :id="fileName"
        :ref="fileName"
        type="checkbox"
        class="mr-2 accent-pink-500"
        :checked="props.checked"
        @input="(event)=>updateChecked((event.target as HTMLInputElement).checked)"
        @click="displayFile"
      />
      {{ fileName }}
    </label>
  </div>
</template>

<style scoped>
.break {
  word-break: normal;
  overflow-wrap: anywhere;
}
</style>
