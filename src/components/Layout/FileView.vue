<script setup lang="ts">
import { GitStatus } from "@types";
import type { PropType } from "vue";

interface Emits {
  (e: "update:checked", checked: boolean): void;
}
const emits = defineEmits<Emits>();
function updateChecked(checked: boolean) {
  console.log(1234, checked);
  emits("update:checked", checked);
}
const props = defineProps({
  fileName: {
    type: String,
    default: null,
  },
  status: {
    type: String as PropType<GitStatus>,
    default: null,
  },
  checked: {
    type: Boolean,
    default: true,
  },
});
function getStatusColor(status: GitStatus) {
  console.log("status color",status)
  if (status == "Modified") return "text-[#b57219]";
  if (status == GitStatus.New) return "text-[#22a81b]";
  if (status == GitStatus.Deleted) return "text-[#bf1b1b]";
  if (status == GitStatus.Unknown) return "text-[#575757]";
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
      type="checkbox"
      :id="fileName"
      class="accent-pink-500"
      :ref="fileName"
      :checked="props.checked"
      @input="(event)=>updateChecked((event.target as HTMLInputElement).checked)"
    />
    <label :for="fileName">{{ fileName }}</label>
  </div>
</template>

<style scoped>
a {
  color: #42b983;
}

label {
  margin: 0 0.5em;
  font-weight: bold;
}

code {
  background-color: #eee;
  padding: 2px 4px;
  border-radius: 4px;
  color: #304455;
}
</style>
