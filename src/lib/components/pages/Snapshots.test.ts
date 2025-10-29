import { describe, expect, it } from 'vitest';
import { formatBytes, formatDate } from '../../utils/format';

// Unit-Tests für Hilfsfunktionen der Snapshots-Seite

describe('Snapshots Page Utils', () => {
  it('formatiert Datum korrekt', () => {
    const date = '2025-10-29T12:34:56Z';
    const formatted = formatDate(date);
    expect(formatted).toMatch(/29\.10\.2025/);
  });

  it('formatiert Bytes korrekt', () => {
    expect(formatBytes(0)).toBe('0 B');
    expect(formatBytes(1024)).toBe('1 KB');
    expect(formatBytes(1048576)).toBe('1 MB');
    expect(formatBytes(1073741824)).toBe('1 GB');
    expect(formatBytes(1536)).toBe('1.5 KB');
  });
});
