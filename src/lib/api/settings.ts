import { invoke } from '@tauri-apps/api/core';

/**
 * API-Wrapper für Settings-Commands (M4.4)
 *
 * Backend-Commands: src-tauri/src/commands/settings.rs
 */

/**
 * App-Einstellungen Interface
 */
export interface AppSettings {
  theme: string; // 'system' | 'light' | 'dark'
  log_level: string; // 'error' | 'warn' | 'info' | 'debug' | 'trace'
  check_updates: boolean;
  max_concurrent_backups: number;
  notifications_enabled: boolean;
  language: string; // 'de' | 'en'
  password_storage: string; // 'system_keychain' | 'in_memory'
  lock_timeout: number; // Minuten
}

/**
 * Lädt die aktuellen App-Einstellungen
 *
 * @returns Promise mit App-Einstellungen
 * @throws Error wenn Laden fehlschlägt
 */
export async function getSettings(): Promise<AppSettings> {
  return await invoke<AppSettings>('get_settings');
}

/**
 * Speichert App-Einstellungen
 *
 * @param settings - Einstellungen zum Speichern
 * @returns Promise (void)
 * @throws Error wenn Speichern fehlschlägt
 */
export async function saveSettings(settings: AppSettings): Promise<void> {
  await invoke('save_settings', { settings });
}

/**
 * Setzt Einstellungen auf Standardwerte zurück
 *
 * @returns Promise mit Standard-Einstellungen
 * @throws Error wenn Reset fehlschlägt
 */
export async function resetSettings(): Promise<AppSettings> {
  return await invoke<AppSettings>('reset_settings');
}

/**
 * Aktualisiert nur das Theme
 *
 * @param theme - Neues Theme ('system', 'light', 'dark')
 * @returns Promise (void)
 * @throws Error wenn Update fehlschlägt
 */
export async function updateTheme(theme: string): Promise<void> {
  await invoke('update_theme', { theme });
}
