<template>
  <div class="flex flex-col bg-[#1e1e2e] h-full">
    <!-- Tabs -->
    <div class="flex bg-[#181825] border-b border-[#313244] overflow-x-auto shrink-0">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        class="px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors cursor-pointer"
        :class="compilerStore.activeOutputTab === tab.id
          ? 'text-[#cdd6f4] border-b-2 border-[#89b4fa]'
          : 'text-[#6c7086] hover:text-[#cdd6f4]'"
        @click="compilerStore.activeOutputTab = tab.id"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-auto">
      <div v-if="!compilerStore.compileResult" class="flex items-center justify-center h-full text-[#6c7086] text-sm">
        Press Compile (Ctrl+Enter) to generate code
      </div>

      <GeneratedCode
        v-else-if="isCodeTab"
        :code="currentCode"
        :language="currentLanguage"
      />

      <CompilerErrors
        v-else-if="compilerStore.activeOutputTab === 'errors'"
        :diagnostics="compilerStore.diagnostics"
      />

      <div v-else-if="compilerStore.activeOutputTab === 'idl'" class="p-3">
        <pre class="text-xs text-[#cdd6f4] whitespace-pre-wrap font-mono">{{ formatIdl }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useCompilerStore } from '@/stores/compiler'
import GeneratedCode from './GeneratedCode.vue'
import CompilerErrors from './CompilerErrors.vue'

const compilerStore = useCompilerStore()

const tabs = [
  { id: 'lib.rs', label: 'lib.rs' },
  { id: 'state.rs', label: 'state.rs' },
  { id: 'instructions.rs', label: 'instructions.rs' },
  { id: 'error.rs', label: 'error.rs' },
  { id: 'events.rs', label: 'events.rs' },
  { id: 'client.ts', label: 'Client (TS)' },
  { id: 'idl', label: 'IDL' },
  { id: 'errors', label: 'Errors' },
]

const codeTabs: Record<string, { field: string; lang: string }> = {
  'lib.rs': { field: 'lib_rs', lang: 'rust' },
  'state.rs': { field: 'state_rs', lang: 'rust' },
  'instructions.rs': { field: 'instructions_rs', lang: 'rust' },
  'error.rs': { field: 'error_rs', lang: 'rust' },
  'events.rs': { field: 'events_rs', lang: 'rust' },
  'client.ts': { field: 'client_ts', lang: 'typescript' },
}

const isCodeTab = computed(() => compilerStore.activeOutputTab in codeTabs)

const currentCode = computed(() => {
  const result = compilerStore.compileResult
  if (!result) return ''
  const tab = codeTabs[compilerStore.activeOutputTab]
  if (!tab) return ''
  return (result as any)[tab.field] ?? ''
})

const currentLanguage = computed(() => {
  return codeTabs[compilerStore.activeOutputTab]?.lang ?? 'text'
})

const formatIdl = computed(() => {
  const idl = compilerStore.compileResult?.idl_json
  if (!idl) return 'No IDL generated'
  try {
    return JSON.stringify(JSON.parse(idl), null, 2)
  } catch {
    return idl
  }
})
</script>
