# Keychain- & Passwortfluss (Stand 2025-11-01)

## Ueberblick

- Dokumentiert den aktuellen Passwort-Workflow zwischen Backend (`AppState`, `keychain::`) und persistenten Konfigurationsdaten.
- Beschreibt Fehlerszenarien sowie empfohlene UI-Reaktionen fuer fehlende oder nicht speicherbare Passwoerter.
- Grundlage fuer weitere Arbeiten in den Bereichen Scheduler, Restore und Portable-Modus.

## Passwort speichern

### Repository anlegen (`init_repository`)

1. Repository wird ueber `crate::rustic::repository::init_repository` initialisiert.
2. Das Passwort wird unmittelbar danach via `crate::keychain::store_password(repo_id, password)` im System-Keychain abgelegt.
3. `AppConfig::add_repository` speichert `password_stored = true` in der Config.
4. `AppState::save_config()` persistiert die aktualisierte Konfiguration.

### Repository wechseln (`switch_repository`)

1. Passwort wird vom Benutzer uebergeben (Unlock-Dialog).
2. Repository wird per `AppState::open_repository_with_password` geoeffnet und gecacht.
3. `store_password` legt das Passwort erneut im Keychain ab (refresh bei geaenderten Credentials).
4. Falls `password_stored` zuvor `false` war, wird es ueber `AppConfig::set_repository_password_stored` -> `save_config()` auf `true` gesetzt.

### Passwort aendern (`change_password`)

1. Altes Passwort wird mittels `crate::rustic::repository::open_repository(path, old_pass)` validiert.
2. Neues Passwort wird im Keychain gespeichert (`store_password`).
3. Seit 2025-11-01 setzt der Command das Config-Flag auf `true` und speichert die Konfiguration direkt.
   - Dadurch entfaellt der bisherige Inkonsistenz-Zustand "Passwort gespeichert, aber Flag = false".

## Passwort laden

### Automatischer Zugriff (`AppState::get_repository`)

1. Prueft Cache und laedt ansonsten Repository-Konfiguration.
2. Wenn `password_stored == true`, wird `keychain::load_password(repo_id)` ausgefuehrt.
3. Rueckgabe-Szenarien:
   - **Erfolg:** Passwort wird fuer Backend-Operationen genutzt.
   - **`PasswordMissing`:**
     - Config-Flag wird auf `false` gesetzt und `save_config()` aufgerufen.
       - Fehler wird an aufrufenden Command weitergereicht, damit das Frontend den Unlock-Dialog zeigt.
   - **`KeychainError`:** Fehler wird zurueckgegeben; Frontend soll Hinweis auf System-Keychain-Probleme anzeigen.
4. Wenn `password_stored == false`, wird sofort `PasswordMissing` erzeugt (kein Keychain-Access notwendig).

## Fehlerfaelle & UX-Empfehlungen

| Fehlerquelle                               | Rueckgabe                                       | Empfohlene UI-Reaktion                                                 |
| ------------------------------------------ | ----------------------------------------------- | ---------------------------------------------------------------------- |
| `KeyringError::NoEntry`                    | `RusticGuiError::PasswordMissing`               | Unlock-Dialog anzeigen, Hinweis auf erforderliche Eingabe              |
| `KeyringError::PlatformFailure` o. ae.     | `RusticGuiError::KeychainError`                 | Fehlerbanner mit Verweis auf System-Keychain/Protokoll                 |
| Config-Flag `password_stored = false`      | `RusticGuiError::PasswordMissing`               | Unlock-Dialog, ggf. Checkbox "Passwort speichern" vorausgewaehlt       |
| `store_password` Fehler (z. B. Permission) | `ErrorDto { code: "KeychainStoreFailed", ... }` | Warn-Toast: "Passwort konnte nicht gespeichert werden" + Debug-Details |

## Auswirkungen auf weitere Bereiche

- **Scheduler:** Bei geplanten Jobs muss vor Start geprueft werden, ob `password_stored` noch `true` ist, sonst Unlock-Anforderung.
- **Portable-Mode:** Fallback-Speicher darf Konfigurations-Updates nicht verhindern; Logging vorhanden (`AppState::emit_portable_status_event`).
  - UI-Hinweis erfolgt über `PortableNotice.svelte` (siehe `docs/mockups/portable_notice.html`).
  - Banner erscheint automatisch, solange `fallback_used` oder `read_only` aktiv ist und lässt sich temporär ausblenden.
- **Restore/Backup Commands:** Nutzen `AppState::with_current_repo`, daher erscheinen `PasswordMissing`-Fehler direkt im Frontend.

## Bekannte Luecken

- UI-Dialoge speichern bisher nicht explizit das Config-Flag; Implementierung folgt mit der Svelte-Forms-Ueberarbeitung.
- Tests fuer Keychain-Workflows fehlen aufgrund fehlender CI-Keychain (siehe TODO in `keychain::tests`).
- Scheduler/Restore-Schnittstellen muessen beim Auftreten von `PasswordMissing` Events anstossen (Follow-Up Task).

## Referenzen

- Backend-Code: `src-tauri/src/commands/repository.rs`, `src-tauri/src/state.rs`, `src-tauri/src/keychain/mod.rs`
- Fehler-Typen: `src-tauri/src/error.rs`
- Portable-Store-Dokumentation: `docs/features/location-picker-implementation.md`
