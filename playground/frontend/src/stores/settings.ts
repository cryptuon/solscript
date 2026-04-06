import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import type { NetworkId } from '@/types/deployment'

const SETTINGS_KEY = 'solscript-settings'

interface Settings {
  fontSize: number
  theme: 'dark' | 'light'
  autoCompile: boolean
  network: NetworkId
  minimap: boolean
  wordWrap: boolean
}

const defaultSettings: Settings = {
  fontSize: 14,
  theme: 'dark',
  autoCompile: true,
  network: 'devnet',
  minimap: false,
  wordWrap: false,
}

function loadSettings(): Settings {
  try {
    const stored = localStorage.getItem(SETTINGS_KEY)
    if (stored) return { ...defaultSettings, ...JSON.parse(stored) }
  } catch {}
  return { ...defaultSettings }
}

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<Settings>(loadSettings())

  watch(settings, (val) => {
    localStorage.setItem(SETTINGS_KEY, JSON.stringify(val))
  }, { deep: true })

  function update(partial: Partial<Settings>) {
    settings.value = { ...settings.value, ...partial }
  }

  return {
    settings,
    update,
  }
})
