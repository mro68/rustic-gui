import { invoke } from '@tauri-apps/api/core';
import type { RetentionPolicy } from '$lib/types';

/**
 * API-Wrapper für Retention-Policy Commands
 * Task 4.3: Retention-Policy Apply
 */

/**
 * Preview-Ergebnis einer Retention-Policy
 */
export interface RetentionPolicyPreview {
  /** Snapshot-IDs die behalten werden */
  snapshots_to_keep: string[];
  /** Snapshot-IDs die gelöscht würden */
  snapshots_to_delete: string[];
}

// Re-export RetentionPolicy für Convenience
export type { RetentionPolicy } from '$lib/types';

/**
 * Simuliert eine Retention-Policy (Dry-Run).
 *
 * Gibt zurück welche Snapshots behalten/gelöscht würden, OHNE tatsächlich zu löschen.
 *
 * @param policy - Retention-Policy (keep_last, keep_daily, etc.)
 * @returns Promise mit Preview (to_keep, to_delete)
 * @throws Error wenn Policy-Anwendung fehlschlägt
 */
export async function previewRetentionPolicy(
  policy: RetentionPolicy
): Promise<RetentionPolicyPreview> {
  return await invoke<RetentionPolicyPreview>('preview_retention_policy', { policy });
}

/**
 * Wendet eine Retention-Policy an und löscht Snapshots.
 *
 * ⚠️ **ACHTUNG**: Diese Funktion löscht WIRKLICH Snapshots!
 *
 * @param policy - Retention-Policy
 * @returns Promise mit Liste der gelöschten Snapshot-IDs
 * @throws Error wenn Policy-Anwendung fehlschlägt
 */
export async function applyRetentionPolicy(policy: RetentionPolicy): Promise<string[]> {
  return await invoke<string[]>('apply_retention_policy', { policy });
}
