export type NetworkId = 'devnet' | 'testnet' | 'mainnet-beta'

export interface NetworkConfig {
  id: NetworkId
  label: string
  endpoint: string
  explorerUrl: string
}

export const NETWORKS: Record<NetworkId, NetworkConfig> = {
  'devnet': {
    id: 'devnet',
    label: 'Devnet',
    endpoint: 'https://api.devnet.solana.com',
    explorerUrl: 'https://explorer.solana.com/?cluster=devnet',
  },
  'testnet': {
    id: 'testnet',
    label: 'Testnet',
    endpoint: 'https://api.testnet.solana.com',
    explorerUrl: 'https://explorer.solana.com/?cluster=testnet',
  },
  'mainnet-beta': {
    id: 'mainnet-beta',
    label: 'Mainnet',
    endpoint: 'https://api.mainnet-beta.solana.com',
    explorerUrl: 'https://explorer.solana.com',
  },
}

export type DeployStatus =
  | 'idle'
  | 'building'
  | 'uploading'
  | 'deploying'
  | 'confirming'
  | 'done'
  | 'error'

export interface DeployResult {
  programId: string
  signature: string
  explorerUrl: string
}
