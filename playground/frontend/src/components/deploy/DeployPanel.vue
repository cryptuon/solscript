<template>
  <div class="p-4">
    <h3 class="text-sm font-medium text-[#cdd6f4] mb-3">Deploy Contract</h3>

    <div v-if="!deploymentStore.connected" class="text-sm text-[#6c7086]">
      Connect a wallet to deploy your contract.
    </div>

    <div v-else class="space-y-3">
      <div class="text-sm text-[#a6adc8]">
        <span class="text-[#6c7086]">Network:</span> {{ deploymentStore.networkConfig.label }}
      </div>
      <div class="text-sm text-[#a6adc8]">
        <span class="text-[#6c7086]">Wallet:</span>
        {{ deploymentStore.publicKey?.slice(0, 8) }}...{{ deploymentStore.publicKey?.slice(-8) }}
      </div>

      <button
        class="w-full px-3 py-2 text-sm font-medium rounded bg-[#cba6f7] text-[#1e1e2e] hover:bg-[#cba6f7]/80 transition-colors cursor-pointer disabled:opacity-50"
        :disabled="!compilerStore.compileResult?.success || deploymentStore.deployStatus !== 'idle'"
        @click="handleDeploy"
      >
        {{ deployButtonText }}
      </button>

      <!-- Progress -->
      <div v-if="deploymentStore.deployStatus !== 'idle'" class="text-xs text-[#6c7086]">
        Status: {{ deploymentStore.deployStatus }}
      </div>

      <!-- Result -->
      <div v-if="deploymentStore.deployResult" class="p-2 bg-[#313244] rounded text-sm">
        <p class="text-[#a6e3a1]">Deployed successfully!</p>
        <p class="text-[#6c7086] mt-1 font-mono text-xs break-all">
          Program ID: {{ deploymentStore.deployResult.programId }}
        </p>
        <a
          :href="deploymentStore.deployResult.explorerUrl"
          target="_blank"
          rel="noopener"
          class="text-[#89b4fa] hover:underline text-xs mt-1 inline-block"
        >
          View on Solana Explorer
        </a>
      </div>

      <!-- Error -->
      <div v-if="deploymentStore.deployError" class="p-2 bg-[#f38ba8]/10 rounded text-sm text-[#f38ba8]">
        {{ deploymentStore.deployError }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useCompilerStore } from '@/stores/compiler'
import { useDeploymentStore } from '@/stores/deployment'

const compilerStore = useCompilerStore()
const deploymentStore = useDeploymentStore()

const deployButtonText = computed(() => {
  switch (deploymentStore.deployStatus) {
    case 'building': return 'Building...'
    case 'uploading': return 'Uploading...'
    case 'deploying': return 'Deploying...'
    case 'confirming': return 'Confirming...'
    case 'done': return 'Deployed!'
    case 'error': return 'Retry Deploy'
    default: return 'Deploy to ' + deploymentStore.networkConfig.label
  }
})

async function handleDeploy() {
  deploymentStore.reset()
  // TODO: Implement full deploy flow when backend BPF compilation is available
  deploymentStore.deployError = 'BPF compilation not yet available. Deploy from CLI using the generated Anchor project.'
}
</script>
