/* eslint-env browser */
import { writable } from 'svelte/store';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
  id: number;
  type: ToastType;
  message: string;
  duration?: number;
}

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);
  let idCounter = 1;

  function show(type: ToastType, message: string, duration = 3500) {
    const id = idCounter++;
    update((toasts) => [...toasts, { id, type, message, duration }]);
    // Auto-remove after duration
    // eslint-disable-next-line no-undef
    setTimeout(() => remove(id), duration);
    return id;
  }

  function remove(id: number) {
    update((toasts) => toasts.filter((t) => t.id !== id));
  }

  return {
    subscribe,
    show,
    success: (msg: string, duration?: number) => show('success', msg, duration),
    error: (msg: string, duration?: number) => show('error', msg, duration),
    warning: (msg: string, duration?: number) => show('warning', msg, duration),
    info: (msg: string, duration?: number) => show('info', msg, duration),
    remove,
  };
}

export const toastStore = createToastStore();
