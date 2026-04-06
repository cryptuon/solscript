<template>
  <select
    :value="deploymentStore.network"
    class="px-2 py-1.5 text-sm bg-[#313244] text-[#cdd6f4] rounded border border-[#45475a] focus:border-[#89b4fa] outline-none cursor-pointer"
    @change="deploymentStore.setNetwork(($event.target as HTMLSelectElement).value as any)"
  >
    <option value="devnet">Devnet</option>
    <option value="testnet">Testnet</option>
    <option value="mainnet-beta">Mainnet</option>
  </select>
</template>

<script setup lang="ts">
import { useDeploymentStore } from '@/stores/deployment'
const deploymentStore = useDeploymentStore()
</script>
