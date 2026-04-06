<template>
  <div class="p-3">
    <div class="flex items-center gap-2 mb-2">
      <div
        class="w-2 h-2 rounded-full"
        :class="statusColor"
      />
      <span class="text-sm text-[#cdd6f4]">{{ statusText }}</span>
    </div>

    <div v-if="progress > 0 && progress < 100" class="w-full bg-[#313244] rounded-full h-1.5">
      <div
        class="bg-[#89b4fa] h-1.5 rounded-full transition-all"
        :style="{ width: `${progress}%` }"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  status: string
  progress: number
}>()

const statusColor = computed(() => {
  switch (props.status) {
    case 'done': return 'bg-[#a6e3a1]'
    case 'error': return 'bg-[#f38ba8]'
    default: return 'bg-[#f9e2af] animate-pulse'
  }
})

const statusText = computed(() => {
  switch (props.status) {
    case 'building': return 'Building BPF program...'
    case 'uploading': return `Uploading program data (${props.progress}%)...`
    case 'deploying': return 'Deploying program...'
    case 'confirming': return 'Confirming transaction...'
    case 'done': return 'Deployment complete!'
    case 'error': return 'Deployment failed'
    default: return 'Ready'
  }
})
</script>
