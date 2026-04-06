import type { ParseResult, CheckResult, CompileResult } from '@/types/project'

let wasmModule: any = null
let initPromise: Promise<void> | null = null

export async function initWasm(): Promise<void> {
  if (wasmModule) return
  if (initPromise) return initPromise

  initPromise = (async () => {
    try {
      const wasm = await import('../wasm-pkg/solscript_wasm.js')
      await wasm.default()
      wasmModule = wasm
    } catch (e) {
      console.error('Failed to initialize WASM module:', e)
      throw e
    }
  })()

  return initPromise
}

export function isWasmReady(): boolean {
  return wasmModule !== null
}

export function wasmParse(source: string): ParseResult {
  if (!wasmModule) throw new Error('WASM module not initialized')
  const json = wasmModule.parse(source)
  return JSON.parse(json) as ParseResult
}

export function wasmCheck(source: string): CheckResult {
  if (!wasmModule) throw new Error('WASM module not initialized')
  const json = wasmModule.check(source)
  return JSON.parse(json) as CheckResult
}

export function wasmCompile(source: string): CompileResult {
  if (!wasmModule) throw new Error('WASM module not initialized')
  const json = wasmModule.compile(source)
  return JSON.parse(json) as CompileResult
}
