/**
 * Formatiert ein ISO-Datum als deutsche Datum/Zeit-String.
 * @param dateString ISO-String
 * @returns Formatierter String
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
 * Formatiert eine Byte-Anzahl als lesbaren String (B, KB, MB, GB, TB)
 * @param bytes Anzahl Bytes
 * @returns Formatierter String
 */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}
