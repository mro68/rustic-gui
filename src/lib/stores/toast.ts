/* eslint-env browser */
import { writable } from 'svelte/store';

/**
 * Toast-Typ-Enum.
 */
export type ToastType = 'success' | 'error' | 'warning' | 'info';

/**
 * Toast-Notification-Objekt.
 */
export interface Toast {
  /** Eindeutige ID */
  id: number;
  /** Toast-Typ (success, error, warning, info) */
  type: ToastType;
  /** Nachrichtentext */
  message: string;
  /** Anzeigedauer in Millisekunden (default: 3500) */
  duration?: number;
}

/**
 * Erstellt den Toast-Store mit Show/Remove-Methoden.
 *
 * @returns Toast-Store mit subscribe, show, success, error, warning, info, remove
 */
function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);
  let idCounter = 1;

  /**
   * Zeigt eine Toast-Notification an.
   *
   * @param type - Toast-Typ
   * @param message - Nachrichtentext
   * @param duration - Anzeigedauer in ms (default: 3500)
   * @returns Toast-ID
   */
  function show(type: ToastType, message: string, duration = 3500) {
    const id = idCounter++;
    update((toasts) => [...toasts, { id, type, message, duration }]);
    // Auto-remove after duration
    // eslint-disable-next-line no-undef
    setTimeout(() => remove(id), duration);
    return id;
  }

  /**
   * Entfernt eine Toast-Notification.
   *
   * @param id - Toast-ID
   */
  function remove(id: number) {
    update((toasts) => toasts.filter((t) => t.id !== id));
  }

  return {
    subscribe,
    show,
    /** Zeigt eine Success-Toast an */
    success: (msg: string, duration?: number) => show('success', msg, duration),
    /** Zeigt eine Error-Toast an */
    error: (msg: string, duration?: number) => show('error', msg, duration),
    /** Zeigt eine Warning-Toast an */
    warning: (msg: string, duration?: number) => show('warning', msg, duration),
    /** Zeigt eine Info-Toast an */
    info: (msg: string, duration?: number) => show('info', msg, duration),
    remove,
  };
}

/**
 * Globaler Toast-Store für Benachrichtigungen.
 *
 * @example
 * ```typescript
 * import { toastStore } from '$lib/stores/toast';
 *
 * toastStore.success('Backup erfolgreich!');
 * toastStore.error('Fehler beim Speichern');
 * toastStore.warning('Warnung: Speicherplatz niedrig');
 * toastStore.info('Info: Update verfügbar');
 *
 * // Mit custom duration
 * toastStore.success('Nachricht', 5000);
 * ```
 */
export const toastStore = createToastStore();
