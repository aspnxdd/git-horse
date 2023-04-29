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
  const status = props.status.toLowerCase();
  return `border-gitstatus-${status} text-gitstatus-${status} bg-gitstatus-${status}/20`;
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
      class="mr-2 hover:text-text-hover flex justify-center"
    >
      <input
        v-if="isInput"
        :id="fileName"
        :ref="fileName"
        type="checkbox"
        class="accent-pink-500 mr-2"
        :checked="props.checked"
        @input="(event)=>updateChecked((event.target as HTMLInputElement).checked)"
        @click="displayFile"
      />
      {{ fileName }}
    </label>
  </div>
</template>

<style scoped></style>
