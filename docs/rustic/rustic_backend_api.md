# rustic_backend 0.5.3 – API-Referenz

## Haupttypen & Methoden

### BackendOptions

Konfiguriert die Backends (lokal, S3, rclone, etc.).

```rust
pub struct BackendOptions {
    pub repository: Option<String>,
    // ... weitere Backend-spezifische Felder ...
}

impl BackendOptions {
    pub fn to_backends(&self) -> RusticResult<RepositoryBackends>
    // ... weitere Methoden ...
}
```

---

### SupportedBackend

Enum für alle unterstützten Backend-Typen.

```rust
pub enum SupportedBackend {
    Local,
    OpenDAL,
    Rclone,
    Rest,
    // ... weitere Varianten ...
}
```

---

### Backend-Typen

- `LocalBackend`
- `OpenDALBackend`
- `RcloneBackend`
- `RestBackend`

Alle mit eigenen `new(...)`-Konstruktoren, z.B.:

```rust
impl LocalBackend {
    pub fn new(/* ... */) -> RusticResult<Self>
}
impl OpenDALBackend {
    pub fn new(path: impl AsRef<str>, options: BTreeMap<String, String>) -> RusticResult<Self>
}
impl RcloneBackend {
    pub fn new(url: impl AsRef<str>, options: BTreeMap<String, String>) -> RusticResult<Self>
}
impl RestBackend {
    pub fn new(/* ... */) -> RusticResult<Self>
}
```

---

### Weitere relevante Typen

- `BackendChoice`: Trait für Backend-Auswahl.
- `BackendLocation`: Hilfstyp für Backend-Pfade.
- `location_to_type_and_path`: Hilfsfunktion zur Backend-Erkennung.

---

## Beispiel: Backend-Initialisierung

```rust
use rustic_backend::BackendOptions;

let backends = BackendOptions::default()
    .repository("/pfad/zum/repo")
    .to_backends()
    .unwrap();
```

---

**Alle Methoden liefern in der Regel ein `RusticResult<T>`, das Fehlerbehandlung nach rustic-Standard ermöglicht.**

Für Details zu einzelnen Feldern/Enums bitte im jeweiligen Quellcode nachschlagen (`src/choose.rs`, `src/local.rs`, `src/opendal.rs`, `src/rclone.rs`, `src/rest.rs`).
