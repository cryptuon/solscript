import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { NetworkId, DeployStatus, DeployResult } from '@/types/deployment'
import { NETWORKS } from '@/types/deployment'
import * as wallet from '@/services/wallet'

export const useDeploymentStore = defineStore('deployment', () => {
  const network = ref<NetworkId>('devnet')
  const deployStatus = ref<DeployStatus>('idle')
  const deployResult = ref<DeployResult | null>(null)
  const deployError = ref<string | null>(null)
  const deployProgress = ref(0)

  const { connected, publicKey, providerName } = wallet.useWalletState()

  const networkConfig = computed(() => NETWORKS[network.value])

  async function connectWallet(walletName: string) {
    try {
      await wallet.connectWallet(walletName)
    } catch (e: any) {
      deployError.value = e.message
    }
  }

  async function disconnectWallet() {
    await wallet.disconnectWallet()
  }

  function setNetwork(id: NetworkId) {
    network.value = id
  }

  function reset() {
    deployStatus.value = 'idle'
    deployResult.value = null
    deployError.value = null
    deployProgress.value = 0
  }

  return {
    network,
    networkConfig,
    deployStatus,
    deployResult,
    deployError,
    deployProgress,
    connected,
    publicKey,
    providerName,
    connectWallet,
    disconnectWallet,
    setNetwork,
    reset,
  }
})
