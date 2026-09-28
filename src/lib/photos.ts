import { invoke } from '@tauri-apps/api/core'

export interface Photo {
  id: string
  name: string
  width: number
  height: number
  modifiedMs: number
}

export const listPhotos = (): Promise<Photo[]> => invoke<Photo[]>('list_photos')

export const thumbUrl = (id: string): string => `gal://localhost/thumb/${id}`
export const fullUrl = (id: string): string => `gal://localhost/full/${id}`
