import { writable } from 'svelte/store';

/**
 * Store für UI-Einstellungen (Theme, Sprache, etc.)
 * - theme: 'dark' | 'light'
 * - language: string (z.B. 'de', 'en')
 */

const _theme = writable<'dark' | 'light'>('dark');
const _language = writable<string>('de');

export const theme = { subscribe: _theme.subscribe };
export const language = { subscribe: _language.subscribe };

export function setTheme(val: 'dark' | 'light') {
  _theme.set(val);
}

export function setLanguage(val: string) {
  _language.set(val);
}

export function resetSettings() {
  _theme.set('dark');
  _language.set('de');
}
