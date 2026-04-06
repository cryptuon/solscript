<template>
  <aside class="flex flex-col bg-[#181825] border-r border-[#313244] w-60 shrink-0 overflow-hidden">
    <!-- Section tabs -->
    <div class="flex border-b border-[#313244]">
      <button
        class="flex-1 py-2 text-xs font-medium uppercase tracking-wider transition-colors cursor-pointer"
        :class="activeTab === 'files' ? 'text-[#cdd6f4] border-b-2 border-[#89b4fa]' : 'text-[#6c7086] hover:text-[#cdd6f4]'"
        @click="activeTab = 'files'"
      >
        Files
      </button>
      <button
        class="flex-1 py-2 text-xs font-medium uppercase tracking-wider transition-colors cursor-pointer"
        :class="activeTab === 'examples' ? 'text-[#cdd6f4] border-b-2 border-[#89b4fa]' : 'text-[#6c7086] hover:text-[#cdd6f4]'"
        @click="activeTab = 'examples'"
      >
        Examples
      </button>
    </div>

    <div class="flex-1 overflow-y-auto">
      <!-- File Tree -->
      <div v-if="activeTab === 'files'" class="p-2">
        <div class="flex items-center justify-between mb-2">
          <span class="text-xs font-medium text-[#6c7086] uppercase tracking-wider">Projects</span>
          <button
            class="text-xs text-[#89b4fa] hover:text-[#b4befe] cursor-pointer"
            @click="showNewProject = true"
          >+ New</button>
        </div>

        <!-- New Project Input -->
        <div v-if="showNewProject" class="mb-2">
          <input
            v-model="newProjectName"
            class="w-full px-2 py-1 text-sm bg-[#313244] text-[#cdd6f4] rounded border border-[#45475a] focus:border-[#89b4fa] outline-none"
            placeholder="Project name"
            @keyup.enter="createProject"
            @keyup.escape="showNewProject = false"
            ref="newProjectInput"
          />
        </div>

        <!-- Project List -->
        <div v-for="project in projectStore.projects" :key="project.id" class="mb-1">
          <button
            class="w-full text-left px-2 py-1.5 text-sm rounded transition-colors cursor-pointer"
            :class="project.id === projectStore.activeProjectId
              ? 'bg-[#313244] text-[#cdd6f4]'
              : 'text-[#a6adc8] hover:bg-[#313244]/50'"
            @click="projectStore.selectProject(project.id)"
          >
            <div class="flex items-center justify-between">
              <span class="truncate">{{ project.name }}</span>
              <span
                class="text-[#6c7086] hover:text-[#f38ba8] text-xs cursor-pointer"
                @click.stop="projectStore.removeProject(project.id)"
              >&times;</span>
            </div>
          </button>

          <!-- Files under active project -->
          <div v-if="project.id === projectStore.activeProjectId" class="ml-3 mt-1">
            <button
              v-for="file in projectStore.files"
              :key="file.id"
              class="w-full text-left px-2 py-1 text-sm rounded transition-colors cursor-pointer"
              :class="file.id === editorStore.activeFileId
                ? 'text-[#89b4fa]'
                : 'text-[#6c7086] hover:text-[#cdd6f4]'"
              @click="editorStore.openFile(file.id, file.name, file.content)"
            >
              {{ file.name }}
            </button>
          </div>
        </div>

        <p v-if="projectStore.projects.length === 0" class="text-xs text-[#6c7086] px-2 py-4 text-center">
          No projects yet. Create one or load an example.
        </p>
      </div>

      <!-- Examples -->
      <ExampleBrowser v-if="activeTab === 'examples'" />
    </div>
  </aside>
</template>

<script setup lang="ts">
import { ref, nextTick } from 'vue'
import { useProjectStore } from '@/stores/project'
import { useEditorStore } from '@/stores/editor'
import ExampleBrowser from '@/components/project/ExampleBrowser.vue'

const projectStore = useProjectStore()
const editorStore = useEditorStore()

const activeTab = ref<'files' | 'examples'>('files')
const showNewProject = ref(false)
const newProjectName = ref('')
const newProjectInput = ref<HTMLInputElement | null>(null)

async function createProject() {
  if (!newProjectName.value.trim()) return
  await projectStore.createProject(newProjectName.value.trim())
  newProjectName.value = ''
  showNewProject.value = false
}
</script>
