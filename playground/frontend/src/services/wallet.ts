import { ref, computed } from 'vue'

export interface WalletProvider {
  name: string
  icon: string
  connect: () => Promise<string>
  disconnect: () => Promise<void>
  signTransaction: (tx: any) => Promise<any>
  signAllTransactions: (txs: any[]) => Promise<any[]>
  publicKey: string | null
}

const connected = ref(false)
const publicKey = ref<string | null>(null)
const providerName = ref<string | null>(null)

function getPhantomProvider(): any {
  if (typeof window !== 'undefined' && 'solana' in window) {
    const provider = (window as any).solana
    if (provider?.isPhantom) return provider
  }
  return null
}

function getSolflareProvider(): any {
  if (typeof window !== 'undefined' && 'solflare' in window) {
    return (window as any).solflare
  }
  return null
}

export function getAvailableWallets(): { name: string; available: boolean }[] {
  return [
    { name: 'Phantom', available: !!getPhantomProvider() },
    { name: 'Solflare', available: !!getSolflareProvider() },
  ]
}

export async function connectWallet(walletName: string): Promise<void> {
  let provider: any

  if (walletName === 'Phantom') {
    provider = getPhantomProvider()
    if (!provider) throw new Error('Phantom wallet not found. Please install it.')
  } else if (walletName === 'Solflare') {
    provider = getSolflareProvider()
    if (!provider) throw new Error('Solflare wallet not found. Please install it.')
  } else {
    throw new Error(`Unknown wallet: ${walletName}`)
  }

  const resp = await provider.connect()
  publicKey.value = resp.publicKey.toString()
  connected.value = true
  providerName.value = walletName
}

export async function disconnectWallet(): Promise<void> {
  const provider = providerName.value === 'Phantom' ? getPhantomProvider() : getSolflareProvider()
  if (provider) {
    await provider.disconnect()
  }
  connected.value = false
  publicKey.value = null
  providerName.value = null
}

export function getWalletProvider(): any {
  if (providerName.value === 'Phantom') return getPhantomProvider()
  if (providerName.value === 'Solflare') return getSolflareProvider()
  return null
}

export function useWalletState() {
  return {
    connected: computed(() => connected.value),
    publicKey: computed(() => publicKey.value),
    providerName: computed(() => providerName.value),
  }
}
