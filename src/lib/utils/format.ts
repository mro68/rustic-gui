/**
 * Formatiert ein ISO-Datum als deutsche Datum/Zeit-String.
 *
 * @param dateString - ISO-String (z.B. "2024-10-26T12:30:00Z")
 * @returns Formatierter String (z.B. "26.10.2024, 12:30:00")
 * @throws Error wenn dateString ungültig ist (Invalid Date)
 */
export function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleString('de-DE', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
}

/**
 * Formatiert eine Byte-Anzahl als lesbaren String (B, KB, MB, GB, TB).
 *
 * @param bytes - Anzahl Bytes (Zahl >= 0)
 * @returns Formatierter String (z.B. "1.5 MB", "3.2 GB")
 * @throws Error wenn bytes negativ ist
 */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}
