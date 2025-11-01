import { writable } from 'svelte/store';
import type { PortableStoreStatus } from '$lib/types';

const _portableStatus = writable<PortableStoreStatus | null>(null);

export const portableStatus = { subscribe: _portableStatus.subscribe };

/**
 * Aktualisiert den Store mit dem zuletzt empfangenen PortableStoreStatus.
 */
export function setPortableStatus(status: PortableStoreStatus | null): void {
  _portableStatus.set(status);
}
