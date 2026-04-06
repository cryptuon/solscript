import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { WasmDiagnostic, CompileResult } from '@/types/project'
import { wasmCheck, wasmCompile, isWasmReady } from '@/services/wasm'

export type CompileStatus =
  | 'idle'
  | 'checking'
  | 'compiling'
  | 'building'
  | 'done'
  | 'error'

export const useCompilerStore = defineStore('compiler', () => {
  const status = ref<CompileStatus>('idle')
  const diagnostics = ref<WasmDiagnostic[]>([])
  const compileResult = ref<CompileResult | null>(null)
  const activeOutputTab = ref('lib.rs')

  const hasErrors = computed(() => diagnostics.value.some(d => d.severity === 'error'))

  let checkTimeout: ReturnType<typeof setTimeout> | null = null

  function checkSource(source: string) {
    if (!isWasmReady()) return

    if (checkTimeout) clearTimeout(checkTimeout)
    checkTimeout = setTimeout(() => {
      try {
        status.value = 'checking'
        const result = wasmCheck(source)
        diagnostics.value = result.diagnostics
        status.value = result.success ? 'idle' : 'error'
      } catch (e) {
        console.error('Check failed:', e)
        status.value = 'error'
      }
    }, 300)
  }

  function compile(source: string) {
    if (!isWasmReady()) return

    try {
      status.value = 'compiling'
      const result = wasmCompile(source)
      compileResult.value = result
      diagnostics.value = result.diagnostics

      if (result.success) {
        status.value = 'done'
        activeOutputTab.value = 'lib.rs'
      } else {
        status.value = 'error'
      }
    } catch (e) {
      console.error('Compile failed:', e)
      status.value = 'error'
    }
  }

  function clearResults() {
    compileResult.value = null
    diagnostics.value = []
    status.value = 'idle'
  }

  return {
    status,
    diagnostics,
    compileResult,
    activeOutputTab,
    hasErrors,
    checkSource,
    compile,
    clearResults,
  }
})
