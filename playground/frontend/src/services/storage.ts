import Dexie, { type Table } from 'dexie'
import type { Project, ProjectFile } from '@/types/project'

class PlaygroundDB extends Dexie {
  projects!: Table<Project, string>
  files!: Table<ProjectFile, string>

  constructor() {
    super('solscript-playground')
    this.version(1).stores({
      projects: 'id, name, updatedAt',
      files: 'id, projectId, name',
    })
  }
}

export const db = new PlaygroundDB()

export async function getAllProjects(): Promise<Project[]> {
  return db.projects.orderBy('updatedAt').reverse().toArray()
}

export async function getProject(id: string): Promise<Project | undefined> {
  return db.projects.get(id)
}

export async function saveProject(project: Project): Promise<void> {
  await db.projects.put(project)
}

export async function deleteProject(id: string): Promise<void> {
  await db.transaction('rw', db.projects, db.files, async () => {
    await db.files.where('projectId').equals(id).delete()
    await db.projects.delete(id)
  })
}

export async function getProjectFiles(projectId: string): Promise<ProjectFile[]> {
  return db.files.where('projectId').equals(projectId).toArray()
}

export async function getFile(id: string): Promise<ProjectFile | undefined> {
  return db.files.get(id)
}

export async function saveFile(file: ProjectFile): Promise<void> {
  await db.files.put(file)
  await db.projects.update(file.projectId, { updatedAt: Date.now() })
}

export async function deleteFile(id: string): Promise<void> {
  await db.files.delete(id)
}

export function generateId(): string {
  return crypto.randomUUID()
}
