# Contribution Guidelines – Rustic GUI

> **Willkommen! Wir freuen uns über jeden Beitrag.**
>
> Bitte lies diese Guidelines bevor du einen Pull Request erstellst.

---

## 📋 Inhaltsverzeichnis

- [Code of Conduct](#code-of-conduct)
- [Entwicklungs-Workflow](#entwicklungs-workflow)
- [Code-Style](#code-style)
- [Testing](#testing)
- [Dokumentation](#dokumentation)
- [Pull Requests](#pull-requests)

---

## 📜 Code of Conduct

Wir folgen dem [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Sei freundlich, respektvoll und konstruktiv.

---

## 🔄 Entwicklungs-Workflow

### 1. Issue erstellen (optional, aber empfohlen)

```bash
# Für neue Features
https://github.com/rustic-rs/rustic-gui/issues/new?template=feature_request.md

# Für Bugs
https://github.com/rustic-rs/rustic-gui/issues/new?template=bug_report.md
```

### 2. Fork & Branch erstellen

```bash
# Fork erstellen auf GitHub UI
git clone https://github.com/DEIN-USERNAME/rustic-gui.git
cd rustic-gui

# Upstream hinzufügen
git remote add upstream https://github.com/rustic-rs/rustic-gui.git

# Feature-Branch erstellen
git checkout -b feature/mein-feature
```

### 3. Entwickeln

```bash
# Dependencies installieren
npm install

# Dev-Server starten
npm run tauri dev

# Tests laufen lassen (während Entwicklung)
npm test
```

### 4. Committen (Conventional Commits)

Wir nutzen [Conventional Commits](https://www.conventionalcommits.org/) **in Deutsch**:

**Format:**

```
<type>(<scope>): <Beschreibung>

[optionaler Body]

[optionaler Footer: "Closes #123"]
```

**Types:**

- `feat` – Neues Feature
- `fix` – Bugfix
- `docs` – Nur Dokumentation
- `style` – Code-Formatierung (keine Logik-Änderung)
- `refactor` – Code-Refactoring
- `test` – Tests hinzufügen/ändern
- `chore` – Build-System, Dependencies

**Scopes (Beispiele):**

- `snapshots` – Snapshot-Feature
- `backup` – Backup-Logik
- `ui` – UI-Komponenten
- `backend` – Rust-Backend
- `frontend` – Svelte-Frontend

**Beispiele:**

```bash
git commit -m "feat(snapshots): Tag-Filterung implementiert"
git commit -m "fix(backup): Progress-Event-Handling korrigiert"
git commit -m "docs(readme): Installation-Anleitung ergänzt"
```

### 5. Tests & Checks

```bash
# Alle Tests
npm test
cd src-tauri && cargo test

# Linting
npm run lint
cd src-tauri && cargo clippy

# Format-Check
npm run format:check
cd src-tauri && cargo fmt -- --check
```

### 6. Push & Pull Request

```bash
git push origin feature/mein-feature

# Dann auf GitHub: "Compare & pull request"
```

---

## 🎨 Code-Style

### Generell

- **Deutsch:** Kommentare, Docstrings, UI-Texte, Commit-Messages
- **Englisch:** Code-Elemente (Variablen, Funktionen, Typen)
- **DRY:** Don't Repeat Yourself – Wiederverwendbare Komponenten/Funktionen
- **KISS:** Keep It Simple, Stupid – Einfache Lösungen bevorzugen

Siehe: [.github/instructions/code-style.instructions.md](.github/instructions/code-style.instructions.md)

### Rust (Backend)

```rust
// ✅ Gut
/// Führt einen Backup durch.
///
/// # Arguments
/// * `repo_id` - Repository-ID
/// * `paths` - Zu sichernde Pfade
///
/// # Returns
/// Result mit Backup-ID bei Erfolg
#[tauri::command]
async fn run_backup(
    repo_id: String,
    paths: Vec<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Implementation
}

// ❌ Schlecht
#[tauri::command]
async fn backup(r: String, p: Vec<String>) -> Result<String, String> {
    // Keine Dokumentation, unklar
}
```

**Conventions:**

- Snake_case für Funktionen/Variablen
- PascalCase für Typen/Structs
- Clippy-Warnings beheben (`cargo clippy`)
- Rustfmt verwenden (`cargo fmt`)

### TypeScript (Frontend)

```typescript
// ✅ Gut
/**
 * Lädt alle Snapshots für ein Repository.
 *
 * @param repoId - Repository-ID
 * @returns Promise mit Snapshot-Array
 * @throws Bei Backend-Fehlern
 */
export async function loadSnapshots(repoId: string): Promise<Snapshot[]> {
  return invoke('list_snapshots', { repoId });
}

// ❌ Schlecht
export async function load(id: string) {
  return invoke('list_snapshots', { id }); // Keine Typen, keine Doku
}
```

**Conventions:**

- CamelCase für Funktionen/Variablen
- PascalCase für Typen/Interfaces
- ESLint-Rules beachten (`npm run lint`)
- Prettier verwenden (`npm run format`)

### Svelte (UI)

````svelte
<!-- ✅ Gut -->
<script lang="ts">
  /**
   * Button-Komponente mit verschiedenen Varianten.
   *
   * @component
   * @example
   * ```svelte
   * <Button variant="primary" on:click={handleClick}>
   *   Klick mich
   * </Button>
   * ```
   */

  interface ButtonProps {
    variant?: 'primary' | 'secondary' | 'danger';
    disabled?: boolean;
  }

  let { variant = 'primary', disabled = false, children }: ButtonProps & { children: any } = $props();
</script>

<button class="btn btn-{variant}" {disabled} onclick={() => /* ... */}>
  {@render children()}
</button>

<!-- ❌ Schlecht -->
<script lang="ts">
  let { v, d, children } = $props(); // Unklar
</script>
<button class={v}>{@render children()}</button>
````

**Conventions:**

- Immer TSDoc-Kommentare für Komponenten
- Props mit Interfaces definieren
- **UI muss Mockups in `docs/mockups/` entsprechen**

---

## 🧪 Testing

### Test-Pyramide (70/25/5)

- **70% Unit-Tests:** Einzelne Funktionen isoliert testen
- **25% Integration-Tests:** Zusammenspiel mehrerer Module
- **5% E2E-Tests:** Vollständige User-Workflows

### Frontend (Vitest)

```bash
# Alle Tests
npm test

# Watch-Modus
npm run test:watch

# Coverage
npm run test:coverage
```

**Beispiel:**

```typescript
// src/lib/components/shared/Button.test.ts
import { describe, it, expect } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import Button from './Button.svelte';

describe('Button Component', () => {
  it('should render with text', () => {
    const { getByText } = render(Button, { children: 'Klick mich' });
    expect(getByText('Klick mich')).toBeInTheDocument();
  });

  it('should call onClick handler', async () => {
    let clicked = false;
    const { getByRole } = render(Button, {
      onclick: () => {
        clicked = true;
      },
    });

    await fireEvent.click(getByRole('button'));
    expect(clicked).toBe(true);
  });
});
```

### Backend (Cargo Test)

```bash
cd src-tauri
cargo test
```

**Beispiel:**

```rust
// src-tauri/src/commands/snapshot.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_id_parsing() {
        let id = "abc123def456";
        let result = parse_snapshot_id(id);
        assert!(result.is_ok());
    }
}
```

### Tests sind Pflicht für:

- ✅ Neue Backend-Commands
- ✅ Neue API-Wrapper (Frontend)
- ✅ Shared Components mit Logik
- ✅ Stores mit komplexer State-Management

---

## 📚 Dokumentation

### Code-Dokumentation

- ✅ **TSDoc** für TypeScript-Funktionen
- ✅ **Rustdoc** für Rust-Funktionen/Structs
- ✅ **Svelte-Kommentare** für Komponenten (mit `@component`, `@example`)

### Instructions aktualisieren

Bei neuen Patterns/Features:

1. Prüfe ob Pattern in [.github/instructions/patterns.instructions.md](.github/instructions/patterns.instructions.md) existiert
2. Falls nicht: Füge neues Pattern mit Code-Beispiel hinzu
3. Bei UI-Komponenten: Ergänze [.github/instructions/frontend.instructions.md](.github/instructions/frontend.instructions.md)
4. Bei Backend-Features: Ergänze [.github/instructions/backend.instructions.md](.github/instructions/backend.instructions.md)

### ROADMAP.md & TODO.md

- Nach Feature-Completion: Status in ROADMAP.md aktualisieren (`[ ]` → `[x]`)
- Bei Bugfixes: Ergänze CHANGELOG.md

---

## 🔀 Pull Requests

### Vor dem PR

- [ ] Alle Tests erfolgreich (`npm test` + `cargo test`)
- [ ] Linting ohne Fehler (`npm run lint` + `cargo clippy`)
- [ ] Formatierung korrekt (`npm run format` + `cargo fmt`)
- [ ] Dokumentation aktualisiert (Instructions, ROADMAP, CHANGELOG)
- [ ] Commits folgen Conventional Commits

### PR-Template

Nutze unsere PR-Vorlage oder schreibe:

```markdown
## Beschreibung

Kurze Zusammenfassung der Änderungen.

## Änderungen

- [ ] Backend-Commands hinzugefügt
- [ ] Frontend-UI implementiert
- [ ] Tests geschrieben
- [ ] Dokumentation aktualisiert

## Screenshots (bei UI-Änderungen)

[Screenshot einfügen]

## Related Issues

Closes #123

## Checklist

- [ ] Tests erfolgreich
- [ ] Linting sauber
- [ ] Dokumentation aktualisiert
- [ ] ROADMAP.md Status aktualisiert
```

### Review-Prozess

1. **Automatische Checks:** CI/CD muss grün sein
2. **Code-Review:** Mindestens 1 Approval von Maintainer
3. **Merge:** Squash-Merge bevorzugt

---

## 🙏 Danke!

Vielen Dank für deinen Beitrag! Jeder PR hilft das Projekt besser zu machen.

Bei Fragen:

- **Issues:** [GitHub Issues](https://github.com/rustic-rs/rustic-gui/issues)
- **Discord:** [discord.gg/WRUWENZnzQ](https://discord.gg/WRUWENZnzQ)

**Happy Coding! 🚀**
