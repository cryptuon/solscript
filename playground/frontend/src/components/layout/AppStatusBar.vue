<template>
  <footer class="flex items-center justify-between px-3 py-1 bg-[#181825] border-t border-[#313244] text-xs text-[#6c7086] shrink-0">
    <div class="flex items-center gap-3">
      <span v-if="compilerStore.status === 'idle'" class="text-[#a6e3a1]">Ready</span>
      <span v-else-if="compilerStore.status === 'checking'" class="text-[#f9e2af]">Checking...</span>
      <span v-else-if="compilerStore.status === 'compiling'" class="text-[#89b4fa]">Compiling...</span>
      <span v-else-if="compilerStore.status === 'done'" class="text-[#a6e3a1]">Compiled</span>
      <span v-else-if="compilerStore.status === 'error'" class="text-[#f38ba8]">
        {{ compilerStore.diagnostics.length }} error(s)
      </span>

      <span v-if="editorStore.isDirty" class="text-[#f9e2af]">Modified</span>
    </div>

    <div class="flex items-center gap-3">
      <span v-if="deploymentStore.connected">
        {{ deploymentStore.publicKey?.slice(0, 4) }}...{{ deploymentStore.publicKey?.slice(-4) }}
      </span>
      <span>{{ deploymentStore.networkConfig.label }}</span>
      <span>SolScript Playground</span>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { useCompilerStore } from '@/stores/compiler'
import { useEditorStore } from '@/stores/editor'
import { useDeploymentStore } from '@/stores/deployment'

const compilerStore = useCompilerStore()
const editorStore = useEditorStore()
const deploymentStore = useDeploymentStore()
</script>
