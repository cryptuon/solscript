import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface OpenFile {
  id: string
  name: string
  content: string
  originalContent: string
}

export const useEditorStore = defineStore('editor', () => {
  const openFiles = ref<Map<string, OpenFile>>(new Map())
  const activeFileId = ref<string | null>(null)

  const activeFile = computed(() => {
    if (!activeFileId.value) return null
    return openFiles.value.get(activeFileId.value) ?? null
  })

  const openFileList = computed(() => Array.from(openFiles.value.values()))

  const isDirty = computed(() => {
    if (!activeFile.value) return false
    return activeFile.value.content !== activeFile.value.originalContent
  })

  function openFile(id: string, name: string, content: string) {
    if (!openFiles.value.has(id)) {
      openFiles.value.set(id, {
        id,
        name,
        content,
        originalContent: content,
      })
    }
    activeFileId.value = id
  }

  function closeFile(id: string) {
    openFiles.value.delete(id)
    if (activeFileId.value === id) {
      const remaining = Array.from(openFiles.value.keys())
      activeFileId.value = remaining.length > 0 ? remaining[remaining.length - 1] : null
    }
  }

  function updateContent(content: string) {
    if (!activeFileId.value) return
    const file = openFiles.value.get(activeFileId.value)
    if (file) {
      file.content = content
    }
  }

  function markSaved(id: string, content: string) {
    const file = openFiles.value.get(id)
    if (file) {
      file.originalContent = content
      file.content = content
    }
  }

  return {
    openFiles,
    activeFileId,
    activeFile,
    openFileList,
    isDirty,
    openFile,
    closeFile,
    updateContent,
    markSaved,
  }
})
