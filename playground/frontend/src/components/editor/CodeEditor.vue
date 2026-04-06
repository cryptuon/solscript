<template>
  <div ref="editorContainer" class="w-full h-full" />
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue'
import * as monaco from 'monaco-editor'
import { registerSolScriptLanguage, setDiagnostics } from './MonacoConfig'
import { useEditorStore } from '@/stores/editor'
import { useCompilerStore } from '@/stores/compiler'
import { useSettingsStore } from '@/stores/settings'

const editorContainer = ref<HTMLElement | null>(null)
let editor: monaco.editor.IStandaloneCodeEditor | null = null
let isUpdatingFromStore = false

const editorStore = useEditorStore()
const compilerStore = useCompilerStore()
const settingsStore = useSettingsStore()

onMounted(() => {
  registerSolScriptLanguage()

  if (!editorContainer.value) return

  editor = monaco.editor.create(editorContainer.value, {
    value: editorStore.activeFile?.content ?? '',
    language: 'solscript',
    theme: 'solscript-dark',
    fontSize: settingsStore.settings.fontSize,
    minimap: { enabled: settingsStore.settings.minimap },
    wordWrap: settingsStore.settings.wordWrap ? 'on' : 'off',
    automaticLayout: true,
    scrollBeyondLastLine: false,
    renderLineHighlight: 'line',
    lineNumbers: 'on',
    tabSize: 4,
    insertSpaces: true,
    formatOnPaste: true,
    suggestOnTriggerCharacters: true,
    padding: { top: 8 },
  })

  editor.onDidChangeModelContent(() => {
    if (isUpdatingFromStore) return
    const content = editor!.getValue()
    editorStore.updateContent(content)
    compilerStore.checkSource(content)
  })

  // Keyboard shortcuts
  editor.addAction({
    id: 'solscript-save',
    label: 'Save File',
    keybindings: [monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS],
    run: () => {
      const projectStore = (window as any).__projectStore
      if (projectStore) projectStore.saveCurrentFile()
    },
  })

  editor.addAction({
    id: 'solscript-compile',
    label: 'Compile',
    keybindings: [monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter],
    run: () => {
      const content = editor!.getValue()
      compilerStore.compile(content)
    },
  })
})

onBeforeUnmount(() => {
  editor?.dispose()
})

// Watch for active file changes
watch(() => editorStore.activeFile, (file) => {
  if (!editor || !file) return
  const currentValue = editor.getValue()
  if (currentValue !== file.content) {
    isUpdatingFromStore = true
    editor.setValue(file.content)
    isUpdatingFromStore = false
  }
})

// Watch for diagnostics changes
watch(() => compilerStore.diagnostics, (diagnostics) => {
  if (!editor) return
  setDiagnostics(editor, diagnostics)
}, { deep: true })

// Watch for settings changes
watch(() => settingsStore.settings, (s) => {
  if (!editor) return
  editor.updateOptions({
    fontSize: s.fontSize,
    minimap: { enabled: s.minimap },
    wordWrap: s.wordWrap ? 'on' : 'off',
  })
}, { deep: true })
</script>
