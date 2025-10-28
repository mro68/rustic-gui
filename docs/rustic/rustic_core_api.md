# rustic_core 0.8.0 – API-Referenz

## Haupttypen & Methoden

### RepositoryOptions

Konfiguriert das Öffnen/Erstellen eines Repositories.

```rust
pub struct RepositoryOptions {
    pub password: Option<String>,
    pub password_file: Option<PathBuf>,
    pub password_command: Option<String>,
    // ... weitere Felder ...
}

impl RepositoryOptions {
    pub fn evaluate_password(&self) -> RusticResult<Option<String>>
    pub fn read_password_from_reader(file: &mut impl BufRead) -> RusticResult<String>
    // ... weitere Methoden ...
}
```

---

### Repository<P, S>

Zentrale API für alle Repository-Operationen.

```rust
pub struct Repository<P, S> { /* ... */ }

impl<P, S> Repository<P, S> {
    pub fn new(opts: &RepositoryOptions, backends: &RepositoryBackends) -> RusticResult<Self>
    pub fn open(self) -> RusticResult<Repository<P, OpenStatus>>
    pub fn open_with_password(self, password: &str) -> RusticResult<Repository<P, OpenStatus>>
    pub fn list<T: RepoId>(&self) -> RusticResult<impl Iterator<Item = T>>
    pub fn find_ids<I: RepoId, T: AsRef<str>>(&self, ...)
    pub fn infos_files(&self) -> RusticResult<RepoFileInfos>
    pub fn warm_up(&self, packs: impl ExactSizeIterator<Item = PackId>) -> RusticResult<()>
    pub fn progress_bars(&self) -> &P
    // ... weitere Methoden ...

    // Snapshot-Management
    pub fn get_all_snapshots(&self) -> RusticResult<Vec<SnapshotFile>>
    pub fn get_snapshots<T: AsRef<str>>(&self, ids: &[T]) -> RusticResult<Vec<SnapshotFile>>
    pub fn delete_snapshots(&self, ids: &[SnapshotId]) -> RusticResult<()>
    // ... weitere Snapshot-Methoden ...
}
```

---

### SnapshotFile

Repräsentiert einen Snapshot.

```rust
pub struct SnapshotFile { /* ... */ }

impl SnapshotFile {
    pub fn from_options(opts: &SnapshotOptions) -> RusticResult<Self>
    pub fn add_tags(&mut self, tag_lists: Vec<StringList>) -> bool
    pub fn set_tags(&mut self, tag_lists: Vec<StringList>) -> bool
    pub fn remove_tags(&mut self, tag_lists: &[StringList]) -> bool
    // ... weitere Methoden ...
}
```

---

### SnapshotOptions

Hilfstyp zum Erstellen von Snapshots.

```rust
pub struct SnapshotOptions { /* ... */ }

impl SnapshotOptions {
    pub fn add_tags(mut self, tag: &str) -> RusticResult<Self>
    pub fn to_snapshot(&self) -> RusticResult<SnapshotFile>
    // ... weitere Methoden ...
}
```

---

### SnapshotId

Typalias/Wrapper für Snapshot-IDs (Details in `repofile`).

---

### Weitere relevante Typen

- `RepoFile`, `RepoId`: Traits für Repository-Dateien und IDs.
- `SnapshotGroup`, `SnapshotGroupCriterion`: Für Gruppierung/Filterung von Snapshots.
- `StringList`, `PathList`: Hilfstypen für Tags und Pfade.

---

## Beispiel: Repository- und Snapshot-Workflow

```rust
use rustic_backend::BackendOptions;
use rustic_core::{Repository, RepositoryOptions, SnapshotOptions, SnapshotFile};

let repo_opts = RepositoryOptions::default().password("secret");
let backends = BackendOptions::default()
    .repository("/pfad/zum/repo")
    .to_backends()
    .unwrap();

let repo = Repository::new(&repo_opts, &backends).unwrap().open().unwrap();

// Snapshots listen
let snaps: Vec<SnapshotFile> = repo.get_all_snapshots().unwrap();

// Einzelne Snapshots holen
let snaps = repo.get_snapshots(&["snapshot_id"]).unwrap();

// Snapshots löschen
repo.delete_snapshots(&[snapshot_id]).unwrap();
```

---

**Alle Methoden liefern in der Regel ein `RusticResult<T>`, das Fehlerbehandlung nach rustic-Standard ermöglicht.**

Für Details zu einzelnen Feldern/Enums bitte im jeweiligen Quellcode nachschlagen (`src/lib.rs`, `src/repository.rs`, `src/repofile/snapshotfile.rs`).
