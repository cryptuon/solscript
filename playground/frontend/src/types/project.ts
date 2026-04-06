export interface Project {
  id: string
  name: string
  createdAt: number
  updatedAt: number
}

export interface ProjectFile {
  id: string
  projectId: string
  name: string
  content: string
  createdAt: number
  updatedAt: number
}

export interface WasmDiagnostic {
  severity: 'error' | 'warning'
  message: string
  code: string | null
  start: number
  end: number
  help: string | null
}

export interface ParseResult {
  success: boolean
  diagnostics: WasmDiagnostic[]
}

export interface CheckResult {
  success: boolean
  diagnostics: WasmDiagnostic[]
}

export interface CompileResult {
  success: boolean
  lib_rs: string | null
  state_rs: string | null
  instructions_rs: string | null
  error_rs: string | null
  events_rs: string | null
  client_ts: string | null
  tests_ts: string | null
  idl_json: string | null
  anchor_toml: string | null
  cargo_toml: string | null
  package_json: string | null
  diagnostics: WasmDiagnostic[]
}
