<template>
  <div class="p-2">
    <div v-for="file in files" :key="file.id">
      <button
        class="w-full text-left px-2 py-1.5 text-sm rounded transition-colors cursor-pointer"
        :class="file.id === activeFileId
          ? 'bg-[#313244] text-[#89b4fa]'
          : 'text-[#a6adc8] hover:bg-[#313244]/50'"
        @click="$emit('select', file)"
      >
        {{ file.name }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ProjectFile } from '@/types/project'

defineProps<{
  files: ProjectFile[]
  activeFileId: string | null
}>()

defineEmits<{
  select: [file: ProjectFile]
}>()
</script>
