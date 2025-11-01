/// Backend-Module für verschiedene Storage-Typen
///
/// Dieses Modul kapselt die Integration verschiedener Cloud- und Netzwerk-Backends
/// für rustic Repositories.
pub mod opendal;
pub mod rclone;

pub use opendal::{OpenDALConfig, create_opendal_backend, validate_opendal_config};
pub use rclone::{
    RcloneConfig, RcloneManager, create_rclone_backend, create_sftp_backend, validate_rclone_config,
};
