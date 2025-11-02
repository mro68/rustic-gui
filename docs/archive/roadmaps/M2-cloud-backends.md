# Milestone 2: Cloud-Backend-Integration ☁️

> **OpenDAL + Rclone Support für Cloud-Storage**

**Dauer:** 30h (1 Woche) | **Status:** 0% - BLOCKING Cloud-Support  
**Priorität:** 🟠 HIGH  
**Dependencies:** M1 (Repository init/open muss funktionieren)

---

## Übersicht

**Problem:** Aktuell nur Local-Backend unterstützt. Cloud-Storage (S3, Azure, GCS, Dropbox, etc.) nicht nutzbar.

**Lösung:**

- **OpenDAL-Integration** für native Cloud-Provider (S3, Azure, GCS, B2, Wasabi, MinIO)
- **Rclone-Integration** für 70+ Provider (inkl. SFTP, Google Drive, Dropbox, OneDrive)

**UI Status:** ✅ LocationPickerDialog komplett (Local/Network/Cloud/Recent Tabs)

**Betroffene Dateien:**

- `src-tauri/src/rustic/repository.rs` - Backend-Selection in init/open
- `src-tauri/src/commands/repository.rs` - test_connection Command
- Neue Datei: `src-tauri/src/rustic/backends/opendal.rs`
- Neue Datei: `src-tauri/src/rustic/backends/rclone.rs`

---

## 2.1 OpenDAL-Backend-Integration

**Dauer:** 12h | **Priorität:** 🔴 HIGHEST

### Unterstützte Provider

| Provider                 | Endpoint-Format                 | Credentials          | Priorität  |
| ------------------------ | ------------------------------- | -------------------- | ---------- |
| **AWS S3**               | `s3://bucket/prefix`            | Access Key + Secret  | 🔴 HIGHEST |
| **Azure Blob**           | `azblob://container/prefix`     | Account Key          | 🟠 HIGH    |
| **Google Cloud Storage** | `gs://bucket/prefix`            | Service Account JSON | 🟠 HIGH    |
| **Backblaze B2**         | `b2://bucket/prefix`            | App Key + ID         | 🟡 MEDIUM  |
| **Wasabi**               | `s3://bucket` (custom endpoint) | Access Key + Secret  | 🟡 MEDIUM  |
| **MinIO**                | `s3://bucket` (custom endpoint) | Access Key + Secret  | 🟡 MEDIUM  |

### Tasks

#### Task 2.1.1: OpenDAL-Modul erstellen (3h)

**Datei:** `src-tauri/src/rustic/backends/opendal.rs`

```rust
use rustic_backend::{OpenDALBackend, BackendOptions};
use std::collections::BTreeMap;

pub struct OpenDALConfig {
    pub provider: String,      // "s3", "azblob", "gcs", etc.
    pub endpoint: String,       // URL oder bucket/container Name
    pub access_key: String,
    pub secret_key: String,
    pub region: Option<String>,
    pub extra_options: BTreeMap<String, String>,
}

pub fn create_opendal_backend(config: &OpenDALConfig) -> Result<OpenDALBackend> {
    let mut options = BTreeMap::new();

    match config.provider.as_str() {
        "s3" => {
            options.insert("bucket".to_string(), config.endpoint.clone());
            options.insert("access_key_id".to_string(), config.access_key.clone());
            options.insert("secret_access_key".to_string(), config.secret_key.clone());

            if let Some(region) = &config.region {
                options.insert("region".to_string(), region.clone());
            }
        }
        "azblob" => {
            options.insert("container".to_string(), config.endpoint.clone());
            options.insert("account_name".to_string(), config.access_key.clone());
            options.insert("account_key".to_string(), config.secret_key.clone());
        }
        "gcs" => {
            options.insert("bucket".to_string(), config.endpoint.clone());
            options.insert("credential".to_string(), config.secret_key.clone()); // JSON
        }
        _ => anyhow::bail!("Unsupported OpenDAL provider: {}", config.provider),
    }

    // Extra options mergen
    options.extend(config.extra_options.clone());

    OpenDALBackend::new(&config.provider, options)
        .map_err(|e| anyhow!("OpenDAL Backend creation failed: {}", e))
}
```

**Tests:**

- [ ] S3-Backend-Creation (Mock-Credentials)
- [ ] Azure-Backend-Creation
- [ ] GCS-Backend-Creation

---

#### Task 2.1.2: `init_repository()` um OpenDAL erweitern (4h)

**Datei:** `src-tauri/src/rustic/repository.rs:19-54`

**Änderungen:**

```rust
pub fn init_repository(
    path: &str,
    password: &str,
    backend_type: &str,
    backend_options: Option<serde_json::Value>,
) -> Result<RepositoryDto> {
    // Backend erstellen basierend auf Typ
    let backends = match backend_type {
        "local" => {
            let path = PathBuf::from(path);
            BackendOptions::default()
                .repository(path.to_str().unwrap())
                .to_backends()?
        }
        "s3" | "azblob" | "gcs" | "b2" => {
            // OpenDAL-Config aus backend_options parsen
            let opendal_config: OpenDALConfig = serde_json::from_value(
                backend_options.ok_or_else(|| anyhow!("Backend options required for cloud"))?
            )?;

            let backend = create_opendal_backend(&opendal_config)?;

            // RepositoryBackends mit OpenDAL erstellen
            // TODO: rustic_backend API prüfen für korrekten Weg
            todo!("OpenDAL zu RepositoryBackends hinzufügen")
        }
        "rclone" => {
            // Siehe Task 2.2.2
            todo!("Rclone backend")
        }
        _ => anyhow::bail!("Unsupported backend type: {}", backend_type),
    };

    // Repository::new() wie in M1
    let repo_opts = RepositoryOptions::default().password(password);
    let repo = Repository::new(&repo_opts, &backends)?;

    // ... rest wie in M1
}
```

**Tests:**

- [ ] Init S3-Repository (mit lokalem MinIO-Server)
- [ ] Init Azure-Repository (Azurite Emulator)

---

#### Task 2.1.3: Connection-Test Command (3h)

**Neue Funktion:** `test_repository_connection()`

**Datei:** `src-tauri/src/commands/repository.rs` (neu hinzufügen)

```rust
#[tauri::command]
pub async fn test_repository_connection(
    backend_type: String,
    backend_options: serde_json::Value,
) -> Result<ConnectionTestResult, String> {
    match backend_type.as_str() {
        "s3" | "azblob" | "gcs" => {
            let config: OpenDALConfig = serde_json::from_value(backend_options)
                .map_err(|e| format!("Invalid config: {}", e))?;

            let backend = create_opendal_backend(&config)
                .map_err(|e| format!("Backend creation failed: {}", e))?;

            // Einfacher List-Test
            backend.list_files("/")
                .await
                .map_err(|e| format!("Connection test failed: {}", e))?;

            Ok(ConnectionTestResult {
                success: true,
                message: "Connection successful".to_string(),
                latency_ms: 0, // TODO: Messen
            })
        }
        _ => Err("Unsupported backend".to_string()),
    }
}
```

**UI-Integration:** Bereits vorbereitet in `LocationPickerDialog.svelte` (Connection-Test-Button)

**Tests:**

- [ ] Connection-Test mit lokalem MinIO
- [ ] Connection-Test mit falschen Credentials (muss fehlschlagen)

---

#### Task 2.1.4: Credential-Management (2h)

**Sicherheit:** Credentials NICHT in Config speichern, sondern Keychain nutzen.

**Implementierung:**

```rust
// In src-tauri/src/keychain/mod.rs erweitern

pub fn save_cloud_credentials(
    repo_id: &str,
    provider: &str,
    access_key: &str,
    secret_key: &str,
) -> Result<()> {
    let keyring = KeyringManager::new()?;

    // Composite Key für Keychain
    let access_key_id = format!("{}:{}:access_key", repo_id, provider);
    let secret_key_id = format!("{}:{}:secret_key", repo_id, provider);

    keyring.set_password(&access_key_id, access_key)?;
    keyring.set_password(&secret_key_id, secret_key)?;

    Ok(())
}

pub fn load_cloud_credentials(
    repo_id: &str,
    provider: &str,
) -> Result<(String, String)> {
    let keyring = KeyringManager::new()?;

    let access_key_id = format!("{}:{}:access_key", repo_id, provider);
    let secret_key_id = format!("{}:{}:secret_key", repo_id, provider);

    let access_key = keyring.get_password(&access_key_id)?;
    let secret_key = keyring.get_password(&secret_key_id)?;

    Ok((access_key, secret_key))
}
```

**Config speichert nur:**

- Provider-Typ (s3, azblob, gcs)
- Endpoint/Bucket/Container
- Region
- Keine Secrets!

---

## 2.2 Rclone-Backend-Integration

**Dauer:** 10h | **Priorität:** 🟠 HIGH

### Unterstützte Provider (via Rclone)

Rclone unterstützt 70+ Provider, wichtigste:

- **SFTP** (ersetzt fehlenden nativen SFTP-Support)
- **Google Drive**
- **Dropbox**
- **Microsoft OneDrive**
- **pCloud**
- **Mega**
- **FTP/FTPS**
- **WebDAV**

### Tasks

#### Task 2.2.1: Rclone-Modul erstellen (4h)

**Datei:** `src-tauri/src/rustic/backends/rclone.rs`

```rust
use librclone::{Rclone, RcloneConfig};
use rustic_backend::RcloneBackend;

pub struct RcloneManager {
    rclone: Rclone,
}

impl RcloneManager {
    pub fn new() -> Result<Self> {
        // Prüfe ob rclone binary installiert ist
        std::process::Command::new("rclone")
            .arg("version")
            .output()
            .map_err(|_| anyhow!("rclone not installed. Please install from https://rclone.org"))?;

        Ok(Self {
            rclone: Rclone::new()?,
        })
    }

    pub fn configure_remote(
        &self,
        name: &str,
        provider: &str,
        config: &BTreeMap<String, String>,
    ) -> Result<()> {
        // Rclone-Config schreiben
        let mut rclone_config = RcloneConfig::new();
        rclone_config.set_remote(name, provider, config)?;

        self.rclone.save_config(&rclone_config)?;

        Ok(())
    }

    pub fn list_remotes(&self) -> Result<Vec<String>> {
        self.rclone.list_remotes()
            .map_err(|e| anyhow!("Failed to list remotes: {}", e))
    }
}
```

**Tests:**

- [ ] Rclone-Installation prüfen
- [ ] Remote konfigurieren (Mock)
- [ ] Remotes auflisten

---

#### Task 2.2.2: Rclone-Backend in Repository-Init (3h)

**Datei:** `src-tauri/src/rustic/repository.rs` erweitern

```rust
"rclone" => {
    let rclone_config: RcloneConfig = serde_json::from_value(backend_options.unwrap())?;

    // Remote konfigurieren
    let rclone_mgr = RcloneManager::new()?;
    rclone_mgr.configure_remote(
        &rclone_config.remote_name,
        &rclone_config.provider,
        &rclone_config.options,
    )?;

    // RcloneBackend erstellen
    let backend = RcloneBackend::new(
        &format!("{}:{}", rclone_config.remote_name, rclone_config.path),
        rclone_config.options.clone(),
    )?;

    // Zu RepositoryBackends hinzufügen
    todo!("RcloneBackend zu RepositoryBackends")
}
```

---

#### Task 2.2.3: SFTP-Support via Rclone (3h)

**UI:** LocationPickerDialog Network-Tab bereits vorhanden.

**Implementierung:**

```rust
// SFTP ist nur ein Rclone-Provider
pub fn create_sftp_backend(
    host: &str,
    port: u16,
    user: &str,
    password: &str,
    path: &str,
) -> Result<RcloneBackend> {
    let mut config = BTreeMap::new();
    config.insert("type".to_string(), "sftp".to_string());
    config.insert("host".to_string(), host.to_string());
    config.insert("port".to_string(), port.to_string());
    config.insert("user".to_string(), user.to_string());
    config.insert("pass".to_string(), password.to_string());

    let rclone_mgr = RcloneManager::new()?;
    let remote_name = format!("rustic_sftp_{}", host.replace(".", "_"));

    rclone_mgr.configure_remote(&remote_name, "sftp", &config)?;

    RcloneBackend::new(&format!("{}:{}", remote_name, path), config)
}
```

**Tests:**

- [ ] SFTP-Backend mit lokalem SSH-Server

---

## 2.3 LocationPickerDialog Backend-Integration

**Dauer:** 8h | **Priorität:** 🟡 MEDIUM

### Tasks

#### Task 2.3.1: Connection-Test-Button implementieren (3h)

**Datei:** `src/lib/components/dialogs/LocationPickerDialog.svelte`

**Änderungen:**

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  async function testConnection() {
    testing = true;
    testResult = null;

    try {
      const result = await invoke('test_repository_connection', {
        backendType: activeTab === 'cloud' ? selectedProvider : 'sftp',
        backendOptions: getBackendOptions(),
      });

      testResult = {
        success: true,
        message: 'Verbindung erfolgreich',
      };
    } catch (error) {
      testResult = {
        success: false,
        message: `Fehler: ${error}`,
      };
    } finally {
      testing = false;
    }
  }

  function getBackendOptions() {
    if (activeTab === 'cloud') {
      return {
        provider: selectedProvider, // 's3', 'azblob', 'gcs', etc.
        endpoint: cloudBucket,
        access_key: cloudAccessKey,
        secret_key: cloudSecretKey,
        region: cloudRegion,
      };
    } else if (activeTab === 'network') {
      return {
        protocol: networkProtocol, // 'sftp', 'smb', etc.
        host: networkHost,
        port: networkPort,
        user: networkUser,
        password: networkPassword,
        path: networkPath,
      };
    }
  }
</script>

<button on:click={testConnection} disabled={testing}>
  {#if testing}
    <Icon name="spinner" class="animate-spin" />
    Teste...
  {:else}
    <Icon name="check-circle" />
    Verbindung testen
  {/if}
</button>

{#if testResult}
  <div class="alert" class:success={testResult.success} class:error={!testResult.success}>
    {testResult.message}
  </div>
{/if}
```

---

#### Task 2.3.2: Favoriten-Management (3h)

**Backend-Command:** `save_favorite_location()`, `list_favorite_locations()`

**Config-Struktur:**

```rust
#[derive(Serialize, Deserialize)]
pub struct FavoriteLocation {
    pub id: String,
    pub name: String,
    pub backend_type: String,
    pub backend_config: serde_json::Value, // Ohne Secrets!
    pub last_used: DateTime<Utc>,
}
```

**UI:** Recent-Tab zeigt Favoriten + Letzte Verwendungen

---

#### Task 2.3.3: Credential-Prompt-Integration (2h)

**Flow:**

1. User wählt Cloud-Provider
2. Gibt Credentials ein
3. Klickt "Verbindung testen"
4. Bei Erfolg: "Als Favorit speichern?" Dialog
5. Credentials gehen in Keychain, Config speichert nur Metadata

---

## Zusammenfassung & Deliverables

### Gesamt-Dauer

**30 Stunden (1 Woche Vollzeit)**

### Deliverables Checklist

- [ ] **OpenDAL-Integration für 6 Provider** (S3, Azure, GCS, B2, Wasabi, MinIO)
- [ ] **Rclone-Integration** (SFTP, Google Drive, Dropbox, OneDrive)
- [ ] **Connection-Test funktioniert** (UI + Backend)
- [ ] **Credential-Management sicher** (Keychain, keine Secrets in Config)
- [ ] **Favoriten-System** (Letzte Locations speichern)
- [ ] **Unit-Tests** (min. 40% Coverage für Backend-Module)

### Akzeptanz-Kriterien

| Kriterium                                   | Test               | Status |
| ------------------------------------------- | ------------------ | ------ |
| S3-Repository kann initialisiert werden     | MinIO lokal        | [ ]    |
| SFTP-Repository via Rclone funktioniert     | SSH-Server lokal   | [ ]    |
| Connection-Test erkennt falsche Credentials | UI-Test            | [ ]    |
| Credentials werden in Keychain gespeichert  | Keychain prüfen    | [ ]    |
| Favoriten werden geladen                    | Config-File prüfen | [ ]    |

### Nächste Schritte

**Nach M2:**

- **M3: Job-Scheduler** - Automatisierte Backups auf Cloud-Repos
- **M1+M2 Integration-Tests** - Backup auf S3, Restore von SFTP

---

**[← Zurück zu M1](M1-rustic-core-integration.md)** | **[Zurück zur Roadmap](../../ROADMAP.md)** | **[Weiter zu M3 →](M3-job-scheduler.md)**
