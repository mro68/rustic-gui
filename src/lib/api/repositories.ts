import type { Repository } from '$lib/types/repository.types';
import { invoke } from '@tauri-apps/api/core';

/**
 * API-Wrapper für Repository-Kommandos (Tauri Backend)
 *
 * - initRepository
 * - openRepository
 * - checkConnection
 * - listRepositories
 */

export async function initRepository(path: string, password: string, backendType: string): Promise<void> {
  await invoke('init_repository', { path, password, backendType });
}

export async function openRepository(path: string, password: string): Promise<Repository> {
  return await invoke<Repository>('open_repository', { path, password });
}

export async function checkConnection(path: string): Promise<boolean> {
  return await invoke<boolean>('check_repository', { path });
}

export async function listRepositories(): Promise<Repository[]> {
  return await invoke<Repository[]>('list_repositories');
}
