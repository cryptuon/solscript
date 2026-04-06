<template>
  <div v-if="visible" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" @click.self="$emit('close')">
    <div class="bg-[#1e1e2e] border border-[#313244] rounded-lg p-6 w-96 shadow-xl">
      <h2 class="text-lg font-medium text-[#cdd6f4] mb-4">New Project</h2>

      <input
        v-model="name"
        class="w-full px-3 py-2 bg-[#313244] text-[#cdd6f4] rounded border border-[#45475a] focus:border-[#89b4fa] outline-none text-sm"
        placeholder="Project name"
        @keyup.enter="create"
        ref="inputRef"
      />

      <div class="flex justify-end gap-2 mt-4">
        <button
          class="px-4 py-2 text-sm rounded text-[#6c7086] hover:text-[#cdd6f4] transition-colors cursor-pointer"
          @click="$emit('close')"
        >
          Cancel
        </button>
        <button
          class="px-4 py-2 text-sm rounded bg-[#89b4fa] text-[#1e1e2e] hover:bg-[#89b4fa]/80 transition-colors cursor-pointer"
          @click="create"
        >
          Create
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

defineProps<{
  visible: boolean
}>()

const emit = defineEmits<{
  close: []
  create: [name: string]
}>()

const name = ref('')

function create() {
  if (name.value.trim()) {
    emit('create', name.value.trim())
    name.value = ''
  }
}
</script>
