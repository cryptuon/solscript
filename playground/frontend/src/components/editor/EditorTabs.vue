<template>
  <div class="flex bg-[#181825] border-b border-[#313244] overflow-x-auto">
    <button
      v-for="file in editorStore.openFileList"
      :key="file.id"
      class="flex items-center gap-1.5 px-3 py-1.5 text-sm border-r border-[#313244] whitespace-nowrap cursor-pointer transition-colors"
      :class="file.id === editorStore.activeFileId
        ? 'bg-[#1e1e2e] text-[#cdd6f4]'
        : 'text-[#6c7086] hover:text-[#cdd6f4] hover:bg-[#1e1e2e]/50'"
      @click="editorStore.activeFileId = file.id"
    >
      <span>{{ file.name }}</span>
      <span
        v-if="file.content !== file.originalContent"
        class="w-2 h-2 rounded-full bg-[#f9e2af] inline-block"
        title="Unsaved changes"
      />
      <span
        class="ml-1 text-[#6c7086] hover:text-[#f38ba8] cursor-pointer"
        @click.stop="editorStore.closeFile(file.id)"
      >&times;</span>
    </button>
  </div>
</template>

<script setup lang="ts">
import { useEditorStore } from '@/stores/editor'
const editorStore = useEditorStore()
</script>
