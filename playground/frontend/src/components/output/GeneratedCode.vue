<template>
  <div ref="container" class="w-full h-full" />
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue'
import * as monaco from 'monaco-editor'

const props = defineProps<{
  code: string
  language: string
}>()

const container = ref<HTMLElement | null>(null)
let editor: monaco.editor.IStandaloneCodeEditor | null = null

onMounted(() => {
  if (!container.value) return

  editor = monaco.editor.create(container.value, {
    value: props.code,
    language: props.language,
    theme: 'solscript-dark',
    readOnly: true,
    minimap: { enabled: false },
    automaticLayout: true,
    scrollBeyondLastLine: false,
    lineNumbers: 'on',
    fontSize: 13,
    padding: { top: 8 },
    renderLineHighlight: 'none',
    domReadOnly: true,
  })
})

onBeforeUnmount(() => {
  editor?.dispose()
})

watch(() => props.code, (val) => {
  if (editor) {
    editor.setValue(val)
  }
})

watch(() => props.language, (val) => {
  if (editor) {
    const model = editor.getModel()
    if (model) {
      monaco.editor.setModelLanguage(model, val)
    }
  }
})
</script>
