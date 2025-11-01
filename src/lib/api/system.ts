import type { PortableStoreStatus } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';

/**
 * Liefert den aktuellen Status des portablen Konfigurationsspeichers.
 *
 * @returns Promise mit PortableStoreStatus aus dem Backend
 */
export async function getPortableStoreStatus(): Promise<PortableStoreStatus> {
  return await invoke<PortableStoreStatus>('get_portable_status');
}
