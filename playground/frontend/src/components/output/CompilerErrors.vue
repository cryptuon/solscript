<template>
  <div class="p-3">
    <div v-if="diagnostics.length === 0" class="text-sm text-[#a6e3a1]">
      No errors
    </div>

    <div v-for="(diag, i) in diagnostics" :key="i" class="mb-2 last:mb-0">
      <div class="flex items-start gap-2 p-2 rounded bg-[#313244]/50">
        <span
          class="shrink-0 text-xs font-bold px-1.5 py-0.5 rounded"
          :class="diag.severity === 'error' ? 'bg-[#f38ba8]/20 text-[#f38ba8]' : 'bg-[#f9e2af]/20 text-[#f9e2af]'"
        >
          {{ diag.severity.toUpperCase() }}
        </span>
        <div class="flex-1 min-w-0">
          <p class="text-sm text-[#cdd6f4]">{{ diag.message }}</p>
          <p v-if="diag.help" class="text-xs text-[#6c7086] mt-1">{{ diag.help }}</p>
          <p v-if="diag.code" class="text-xs text-[#6c7086] mt-0.5 font-mono">{{ diag.code }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { WasmDiagnostic } from '@/types/project'

defineProps<{
  diagnostics: WasmDiagnostic[]
}>()
</script>
