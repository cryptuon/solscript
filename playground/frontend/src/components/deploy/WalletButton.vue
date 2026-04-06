<template>
  <div class="relative">
    <button
      v-if="!deploymentStore.connected"
      class="px-3 py-1.5 text-sm font-medium rounded bg-[#89b4fa] text-[#1e1e2e] hover:bg-[#89b4fa]/80 transition-colors cursor-pointer"
      @click="showWalletMenu = !showWalletMenu"
    >
      Connect Wallet
    </button>

    <button
      v-else
      class="px-3 py-1.5 text-sm font-medium rounded bg-[#313244] text-[#cdd6f4] hover:bg-[#45475a] transition-colors cursor-pointer"
      @click="deploymentStore.disconnectWallet()"
    >
      {{ deploymentStore.publicKey?.slice(0, 4) }}...{{ deploymentStore.publicKey?.slice(-4) }}
    </button>

    <!-- Wallet menu dropdown -->
    <div
      v-if="showWalletMenu && !deploymentStore.connected"
      class="absolute right-0 top-full mt-1 bg-[#313244] border border-[#45475a] rounded shadow-lg z-50 min-w-[160px]"
    >
      <button
        v-for="w in wallets"
        :key="w.name"
        class="w-full text-left px-3 py-2 text-sm transition-colors cursor-pointer"
        :class="w.available
          ? 'text-[#cdd6f4] hover:bg-[#45475a]'
          : 'text-[#6c7086] cursor-not-allowed'"
        :disabled="!w.available"
        @click="connect(w.name)"
      >
        {{ w.name }}
        <span v-if="!w.available" class="text-xs text-[#6c7086] ml-1">(not found)</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { useDeploymentStore } from '@/stores/deployment'
import { getAvailableWallets } from '@/services/wallet'

const deploymentStore = useDeploymentStore()
const showWalletMenu = ref(false)
const wallets = ref<{ name: string; available: boolean }[]>([])

onMounted(() => {
  wallets.value = getAvailableWallets()
  document.addEventListener('click', closeMenu)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', closeMenu)
})

function closeMenu(e: Event) {
  const target = e.target as HTMLElement
  if (!target.closest('.relative')) {
    showWalletMenu.value = false
  }
}

async function connect(name: string) {
  await deploymentStore.connectWallet(name)
  showWalletMenu.value = false
}
</script>
