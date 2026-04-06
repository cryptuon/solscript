<template>
  <header class="flex items-center justify-between px-4 py-2 bg-[#181825] border-b border-[#313244] shrink-0">
    <div class="flex items-center gap-3">
      <h1 class="text-lg font-bold text-[#cdd6f4]">
        <span class="text-[#89b4fa]">Sol</span>Script
        <span class="text-sm font-normal text-[#6c7086] ml-1">Playground</span>
      </h1>
    </div>

    <div class="flex items-center gap-2">
      <!-- Compile Button -->
      <button
        class="px-3 py-1.5 text-sm font-medium rounded bg-[#a6e3a1] text-[#1e1e2e] hover:bg-[#a6e3a1]/80 transition-colors cursor-pointer disabled:opacity-50"
        :disabled="compilerStore.status === 'compiling'"
        @click="handleCompile"
      >
        {{ compilerStore.status === 'compiling' ? 'Compiling...' : 'Compile' }}
      </button>

      <!-- Network Selector -->
      <NetworkSelector />

      <!-- Wallet Button -->
      <WalletButton />
    </div>
  </header>
</template>

<script setup lang="ts">
import { useCompilerStore } from '@/stores/compiler'
import { useEditorStore } from '@/stores/editor'
import NetworkSelector from '@/components/deploy/NetworkSelector.vue'
import WalletButton from '@/components/deploy/WalletButton.vue'

const compilerStore = useCompilerStore()
const editorStore = useEditorStore()

function handleCompile() {
  const content = editorStore.activeFile?.content
  if (content) {
    compilerStore.compile(content)
  }
}
</script>
