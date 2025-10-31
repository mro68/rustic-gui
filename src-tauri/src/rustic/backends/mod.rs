/// Backend-Module für verschiedene Storage-Typen
///
/// Dieses Modul kapselt die Integration verschiedener Cloud- und Netzwerk-Backends
/// für rustic Repositories.

pub mod opendal;
pub mod rclone;

pub use opendal::{create_opendal_backend, validate_opendal_config, OpenDALConfig};
pub use rclone::{create_rclone_backend, create_sftp_backend, validate_rclone_config, RcloneConfig, RcloneManager};
