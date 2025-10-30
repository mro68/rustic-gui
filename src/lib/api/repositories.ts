import type { RepositoryDto } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';

/**
 * API-Wrapper für Repository-Kommandos (Tauri Backend)
 *
 * TODO.md: Phase 2 - API-Wrapper ✅ KOMPLETT
 * - initRepository
 * - openRepository
 * - checkRepository
 * - listRepositories
 * - deleteRepository
 * - pruneRepository
 * - changePassword
 *
 * Backend-Commands:
 * - lib.rs (init, open, check_repository_v1)
 * - commands/repository.rs (list, delete, check, prune, change_password)
 *
 * ⚠️ Hinweis: Viele Backend-Implementations sind Stubs (siehe TODO-Kommentare in Rust-Code)
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
    backendOptions,
  });
}

export async function openRepository(path: string, password: string): Promise<RepositoryDto> {
  return await invoke<RepositoryDto>('open_repository', { path, password });
}

// Check repository (v1 - requires path and password)
export async function checkRepositoryV1(path: string, password: string): Promise<RepositoryDto> {
  return await invoke<RepositoryDto>('check_repository_v1', { path, password });
}

// Check repository (v2 - uses stored credentials by ID)
export async function checkRepository(id: string, readData: boolean = false): Promise<string> {
  return await invoke<string>('check_repository', { id, readData });
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

export async function changePassword(id: string, oldPass: string, newPass: string): Promise<void> {
  await invoke('change_password', { id, oldPass, newPass });
}
