import type { RepositoryDto } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';

/**
 * API-Wrapper für Repository-Kommandos (Tauri Backend)
 *
 * - initRepository
 * - openRepository
 * - checkRepository
 * - listRepositories
 * - deleteRepository
 * - pruneRepository
 * - changePassword
 */

export async function initRepository(
  path: string,
  password: string,
  backendType: string,
  backendOptions?: Record<string, unknown>
): Promise<RepositoryDto> {
  return await invoke<RepositoryDto>('init_repository', { 
    path, 
    password, 
    backendType,
    backendOptions
  });
}

export async function openRepository(path: string, password: string): Promise<RepositoryDto> {
  return await invoke<RepositoryDto>('open_repository', { path, password });
}

export async function checkRepository(path: string, password: string): Promise<RepositoryDto> {
  return await invoke<RepositoryDto>('check_repository_v1', { path, password });
}

export async function listRepositories(): Promise<RepositoryDto[]> {
  return await invoke<RepositoryDto[]>('list_repositories');
}

export async function deleteRepository(id: string, deleteData: boolean): Promise<void> {
  await invoke('delete_repository', { id, deleteData });
}

export async function pruneRepository(id: string): Promise<string> {
  return await invoke<string>('prune_repository', { id });
}

export async function changePassword(
  id: string,
  oldPass: string,
  newPass: string
): Promise<void> {
  await invoke('change_password', { id, oldPass, newPass });
}
