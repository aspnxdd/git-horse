<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';

interface Emits {
  (e: "update:checked", checked: boolean): void;
}
const emits = defineEmits<Emits>();
function updateChecked(checked: boolean) {
  emits("update:checked", checked);
}
const props = defineProps({
  fileName: {
    type: String,
    default: null,
  },
  checked: {
    type: Boolean,
    default: true,
  },
});
onUpdated(() => {
  console.log("checked", props.checked);
});
async function getFileStatus(){
    await invoke("get_file_status", {path:props.fileName});
}
</script>

<template>
  <input
    type="checkbox"
    :id="fileName"
    class="accent-pink-500"
    :ref="fileName"
    :checked="props.checked"
    @input="(event)=>updateChecked((event.target as HTMLInputElement).checked)"
  />
  <label :for="fileName">{{ fileName }}</label>
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
