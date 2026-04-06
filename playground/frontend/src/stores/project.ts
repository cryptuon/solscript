import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Project, ProjectFile } from '@/types/project'
import * as storage from '@/services/storage'
import { useEditorStore } from './editor'

export const useProjectStore = defineStore('project', () => {
  const projects = ref<Project[]>([])
  const activeProjectId = ref<string | null>(null)
  const files = ref<ProjectFile[]>([])

  const activeProject = computed(() =>
    projects.value.find(p => p.id === activeProjectId.value) ?? null
  )

  async function loadProjects() {
    projects.value = await storage.getAllProjects()
  }

  async function createProject(name: string, initialSource?: string): Promise<Project> {
    const now = Date.now()
    const project: Project = {
      id: storage.generateId(),
      name,
      createdAt: now,
      updatedAt: now,
    }
    await storage.saveProject(project)

    const file: ProjectFile = {
      id: storage.generateId(),
      projectId: project.id,
      name: `${name.toLowerCase().replace(/\s+/g, '_')}.sol`,
      content: initialSource ?? getDefaultSource(name),
      createdAt: now,
      updatedAt: now,
    }
    await storage.saveFile(file)

    await loadProjects()
    await selectProject(project.id)
    return project
  }

  async function selectProject(id: string) {
    activeProjectId.value = id
    files.value = await storage.getProjectFiles(id)

    const editor = useEditorStore()
    if (files.value.length > 0) {
      const file = files.value[0]
      editor.openFile(file.id, file.name, file.content)
    }
  }

  async function removeProject(id: string) {
    await storage.deleteProject(id)
    if (activeProjectId.value === id) {
      activeProjectId.value = null
      files.value = []
    }
    await loadProjects()
  }

  async function saveCurrentFile() {
    const editor = useEditorStore()
    if (!editor.activeFile) return

    const file = files.value.find(f => f.id === editor.activeFile!.id)
    if (!file) return

    file.content = editor.activeFile.content
    file.updatedAt = Date.now()
    await storage.saveFile(file)
    editor.markSaved(file.id, file.content)
  }

  async function createFile(name: string, content: string = ''): Promise<ProjectFile | null> {
    if (!activeProjectId.value) return null
    const now = Date.now()
    const file: ProjectFile = {
      id: storage.generateId(),
      projectId: activeProjectId.value,
      name,
      content,
      createdAt: now,
      updatedAt: now,
    }
    await storage.saveFile(file)
    files.value = await storage.getProjectFiles(activeProjectId.value)
    return file
  }

  return {
    projects,
    activeProjectId,
    activeProject,
    files,
    loadProjects,
    createProject,
    selectProject,
    removeProject,
    saveCurrentFile,
    createFile,
  }
})

function getDefaultSource(name: string): string {
  return `// ${name} - SolScript Contract

contract ${name.replace(/\s+/g, '')} {
    uint256 public value;

    constructor() {
        value = 0;
    }

    function setValue(uint256 newValue) public {
        value = newValue;
    }

    function getValue() public view returns (uint256) {
        return value;
    }
}
`
}
