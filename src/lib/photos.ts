import { invoke } from '@tauri-apps/api/core'

export interface Photo {
  id: string
  name: string
  width: number
  height: number
  modifiedMs: number
}

export const listPhotos = (): Promise<Photo[]> => invoke<Photo[]>('list_photos')

// WebView2 (Windows) serves custom protocols as http://<scheme>.localhost; macOS/Linux use <scheme>://localhost.
const base = navigator.userAgent.includes('Windows')
  ? 'http://gal.localhost'
  : 'gal://localhost'

export const thumbUrl = (id: string): string => `${base}/thumb/${id}`

export const fullUrl = (id: string): string => `${base}/full/${id}`
