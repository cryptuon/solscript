<template>
  <div class="p-2">
    <p class="text-xs text-[#6c7086] mb-2 px-1">Load an example contract to get started</p>
    <button
      v-for="example in examples"
      :key="example.name"
      class="w-full text-left px-2 py-2 text-sm rounded transition-colors cursor-pointer hover:bg-[#313244]/50 mb-0.5"
      @click="loadExample(example)"
    >
      <div class="text-[#cdd6f4]">{{ example.name }}</div>
      <div class="text-xs text-[#6c7086] mt-0.5 truncate">{{ example.description }}</div>
    </button>
  </div>
</template>

<script setup lang="ts">
import { EXAMPLES, type ExampleContract } from '@/services/examples'
import { useProjectStore } from '@/stores/project'

const examples = EXAMPLES
const projectStore = useProjectStore()

async function loadExample(example: ExampleContract) {
  await projectStore.createProject(example.name, example.source)
}
</script>
