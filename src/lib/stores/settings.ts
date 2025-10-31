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

/**
 * Setzt das UI-Theme.
 *
 * @param val - Theme-Modus ('dark' oder 'light')
 */
export function setTheme(val: 'dark' | 'light'): void {
  _theme.set(val);
}

/**
 * Setzt die UI-Sprache.
 *
 * @param val - Sprachcode (z.B. 'de', 'en')
 */
export function setLanguage(val: string): void {
  _language.set(val);
}

/**
 * Setzt die Einstellungen auf Standardwerte zurück.
 *
 * Theme: 'dark', Language: 'de'
 */
export function resetSettings(): void {
  _theme.set('dark');
  _language.set('de');
}
