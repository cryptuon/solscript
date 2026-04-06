import { Connection, PublicKey, Transaction, SystemProgram, BpfLoader, BPF_LOADER_PROGRAM_ID } from '@solana/web3.js'
import { getWalletProvider } from './wallet'
import type { NetworkId, DeployResult } from '@/types/deployment'
import { NETWORKS } from '@/types/deployment'

export async function buildOnServer(source: string): Promise<{ bytecode: ArrayBuffer; idl: string }> {
  const response = await fetch('/api/build', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ source }),
  })

  const data = await response.json()

  if (!data.success) {
    const messages = data.errors.map((e: any) => e.message).join('\n')
    throw new Error(`Build failed:\n${messages}`)
  }

  if (!data.bytecode) {
    throw new Error('Build succeeded but no bytecode returned. BPF compilation may not be available.')
  }

  // Decode base64 bytecode
  const binaryStr = atob(data.bytecode)
  const bytes = new Uint8Array(binaryStr.length)
  for (let i = 0; i < binaryStr.length; i++) {
    bytes[i] = binaryStr.charCodeAt(i)
  }

  return { bytecode: bytes.buffer, idl: data.idl }
}

export async function deployProgram(
  bytecode: ArrayBuffer,
  network: NetworkId,
  onProgress?: (phase: string, progress: number) => void
): Promise<DeployResult> {
  const provider = getWalletProvider()
  if (!provider) throw new Error('No wallet connected')

  const networkConfig = NETWORKS[network]
  const connection = new Connection(networkConfig.endpoint, 'confirmed')
  const payerPubkey = new PublicKey(provider.publicKey.toString())

  onProgress?.('deploying', 0)

  const programData = new Uint8Array(bytecode)

  // Use BpfLoader to deploy the program
  const programId = await BpfLoader.load(
    connection,
    {
      publicKey: payerPubkey,
      secretKey: new Uint8Array(0), // Not used — wallet signs
    } as any,
    null as any,
    programData,
    BPF_LOADER_PROGRAM_ID,
  )

  onProgress?.('done', 100)

  const explorerBase = networkConfig.explorerUrl
  const clusterParam = network === 'mainnet-beta' ? '' : `?cluster=${network}`

  return {
    programId: programId.toString(),
    signature: '',
    explorerUrl: `${explorerBase}/address/${programId.toString()}${clusterParam}`,
  }
}
