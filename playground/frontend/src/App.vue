<template>
  <div class="flex flex-col h-screen overflow-hidden">
    <AppHeader />

    <div class="flex flex-1 overflow-hidden">
      <!-- Sidebar -->
      <AppSidebar />

      <!-- Main area: Editor + Output -->
      <div class="flex flex-col flex-1 overflow-hidden">
        <!-- Editor -->
        <div class="flex flex-col flex-1 min-h-0">
          <EditorTabs />
          <div class="flex-1 min-h-0">
            <CodeEditor v-if="editorStore.activeFile" />
            <div v-else class="flex items-center justify-center h-full text-[#6c7086]">
              <div class="text-center">
                <p class="text-lg mb-2">Welcome to SolScript Playground</p>
                <p class="text-sm">Create a new project or load an example from the sidebar</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Output Panel (resizable) -->
        <div
          class="border-t border-[#313244] shrink-0"
          :style="{ height: outputHeight + 'px' }"
        >
          <!-- Resize handle -->
          <div
            class="h-1 cursor-row-resize hover:bg-[#89b4fa]/30 transition-colors"
            @mousedown="startResize"
          />
          <div class="h-[calc(100%-4px)] overflow-hidden">
            <OutputPanel />
          </div>
        </div>
      </div>
    </div>

    <AppStatusBar />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import AppHeader from '@/components/layout/AppHeader.vue'
import AppSidebar from '@/components/layout/AppSidebar.vue'
import AppStatusBar from '@/components/layout/AppStatusBar.vue'
import EditorTabs from '@/components/editor/EditorTabs.vue'
import CodeEditor from '@/components/editor/CodeEditor.vue'
import OutputPanel from '@/components/output/OutputPanel.vue'
import { useEditorStore } from '@/stores/editor'
import { useProjectStore } from '@/stores/project'
import { initWasm } from '@/services/wasm'

const editorStore = useEditorStore()
const projectStore = useProjectStore()

// Output panel height (resizable)
const outputHeight = ref(280)
let isResizing = false

function startResize(e: MouseEvent) {
  isResizing = true
  const startY = e.clientY
  const startHeight = outputHeight.value

  const onMouseMove = (e: MouseEvent) => {
    if (!isResizing) return
    const delta = startY - e.clientY
    outputHeight.value = Math.max(100, Math.min(600, startHeight + delta))
  }

  const onMouseUp = () => {
    isResizing = false
    document.removeEventListener('mousemove', onMouseMove)
    document.removeEventListener('mouseup', onMouseUp)
  }

  document.addEventListener('mousemove', onMouseMove)
  document.addEventListener('mouseup', onMouseUp)
}

// Expose project store globally for Monaco keyboard shortcuts
;(window as any).__projectStore = projectStore

onMounted(async () => {
  // Initialize WASM module
  try {
    await initWasm()
    console.log('WASM compiler initialized')
  } catch (e) {
    console.warn('WASM module not available, using backend compilation only')
  }

  // Load projects from IndexedDB
  await projectStore.loadProjects()

  // Auto-open last project if exists
  if (projectStore.projects.length > 0) {
    await projectStore.selectProject(projectStore.projects[0].id)
  }
})
</script>
