# 🔧 File Browser Fix - "Verzeichnis auswählen" Button

## Problem

Der "📁 Pfad hinzufügen" Button in der Backup-Job-Konfiguration öffnete **keinen File-Dialog**, sondern fügte nur eine leere Zeile hinzu.

## Root Cause

1. **PathsTab.svelte**: `addPath()` Funktion hatte keine Dialog-Integration
2. **LocalTab.svelte**: `browseLocalDirectory()` war nur TODO mit `console.warn`
3. **Tauri Permissions**: Dialog-Permissions fehlten in `capabilities/default.json`

## Implementierte Lösung

### 1. LocalTab.svelte (LocationPicker)

```typescript
async function browseLocalDirectory() {
  try {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Repository-Verzeichnis auswählen',
    });

    if (selected && typeof selected === 'string') {
      selectedPath = selected;
    }
  } catch (error) {
    console.error('File browser error:', error);
  }
}

async function browseLocalFile() {
  try {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({
      directory: false,
      multiple: false,
      title: 'Repository-Datei auswählen',
      filters: [
        {
          name: 'Repository-Konfiguration',
          extensions: ['json', 'toml'],
        },
      ],
    });

    if (selected && typeof selected === 'string') {
      selectedPath = selected;
    }
  } catch (error) {
    console.error('File browser error:', error);
  }
}
```

### 2. PathsTab.svelte (Backup-Job Quell-Pfade)

**Neue Funktion:**

```typescript
async function browseDirectory(index: number) {
  try {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Quell-Verzeichnis auswählen',
    });

    if (selected && typeof selected === 'string') {
      sourcePaths[index] = selected;
      sourcePaths = [...sourcePaths]; // Trigger reactivity
    }
  } catch (error) {
    console.error('Directory browser error:', error);
  }
}
```

**UI-Änderungen:**

- Jedes Pfad-Input hat jetzt **Browse-Button** (📁)
- Button "📁 Pfad hinzufügen" → "➕ Weiterer Pfad"
- Layout: `[Input] [📁] [✕]` (Browse + Remove pro Zeile)

### 3. Tauri Permissions (capabilities/default.json)

**Vorher:**

```json
{
  "permissions": ["core:default", "opener:default"]
}
```

**Nachher:**

```json
{
  "permissions": ["core:default", "opener:default", "dialog:allow-open", "dialog:allow-save"]
}
```

## Testing

### LocalTab (Repository hinzufügen)

1. Repository hinzufügen Dialog öffnen
2. Tab "Lokal" auswählen
3. Klick "📁 Verzeichnis wählen"
4. **Erwartung:** Nativer File-Dialog öffnet
5. Verzeichnis auswählen
6. **Erwartung:** Pfad erscheint im Input-Feld

### PathsTab (Backup-Job Konfiguration)

1. Neuen Backup-Job erstellen
2. Tab 2: "Paths & Exclusions"
3. Klick 📁 Button neben Pfad-Input
4. **Erwartung:** Nativer Directory-Dialog öffnet
5. Verzeichnis auswählen
6. **Erwartung:** Pfad wird in Input-Feld eingetragen

### Multi-Path Test

1. Klick "➕ Weiterer Pfad"
2. **Erwartung:** Neue leere Zeile erscheint
3. Klick 📁 bei zweitem Pfad
4. **Erwartung:** Dialog öffnet für zweiten Pfad
5. Beide Pfade sollten unterschiedlich sein

## Dateien geändert

```
✅ src/lib/components/dialogs/LocationPicker/LocalTab.svelte
   - browseLocalDirectory() implementiert
   - browseLocalFile() implementiert

✅ src/lib/components/dialogs/JobDialog/PathsTab.svelte
   - browseDirectory(index) hinzugefügt
   - Browse-Button (📁) zu jedem Pfad-Input
   - Button-Text geändert

✅ src-tauri/capabilities/default.json
   - dialog:allow-open Permission
   - dialog:allow-save Permission
```

## Tauri Dialog API Details

**Package:** `@tauri-apps/plugin-dialog` v2.4.2

**Wichtige Optionen:**

- `directory: true` - Verzeichnis-Auswahl statt Datei
- `multiple: false` - Nur 1 Auswahl
- `title: string` - Dialog-Titel
- `filters: []` - Datei-Filter (nur bei Files)

**Return Type:**

- `string | null` bei `multiple: false`
- `string[] | null` bei `multiple: true`

## Auto-Reload

Die App kompiliert automatisch neu (Vite Hot-Reload + Cargo Watch).

**Nach Speichern:**

1. Frontend: ~1-2 Sekunden (Vite HMR)
2. Backend: ~5-10 Sekunden (Cargo compile)
3. Browser refresh falls Tauri-Permissions geändert

## Known Issues

**Keine!** Die Implementation ist straightforward:

- ✅ TypeScript Types korrekt
- ✅ Error-Handling vorhanden
- ✅ Reactivity funktioniert (`sourcePaths = [...sourcePaths]`)
- ✅ Permissions gesetzt

## Next Steps

**Weitere Dialog-Nutzung:**

- Restore-Dialog: Target-Verzeichnis auswählen
- Settings: Config-Import/Export
- Repository-Management: Backup-Verzeichnis migrieren

---

**Status:** ✅ FIXED  
**Getestet:** Nach App-Reload  
**Version:** 2025-11-02
