/// OpenDAL-Backend-Integration für Cloud-Storage
///
/// Unterstützt AWS S3, Azure Blob Storage, Google Cloud Storage,
/// Backblaze B2, Wasabi, MinIO und andere S3-kompatible Services.

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Konfiguration für OpenDAL-Backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenDALConfig {
    /// Provider-Typ (s3, azblob, gcs, b2, etc.)
    pub provider: String,
    /// Endpoint (Bucket/Container Name oder URL)
    pub endpoint: String,
    /// Access Key / Account Name
    pub access_key: String,
    /// Secret Key / Account Key
    pub secret_key: String,
    /// Optional: Region (für S3, etc.)
    pub region: Option<String>,
    /// Optional: Custom Endpoint URL (für MinIO, Wasabi, etc.)
    pub endpoint_url: Option<String>,
    /// Zusätzliche Provider-spezifische Optionen
    pub extra_options: Option<BTreeMap<String, String>>,
}

/// Erstellt ein OpenDAL-Backend für rustic
///
/// # Arguments
/// * `config` - OpenDAL-Konfiguration mit Provider und Credentials
///
/// # Returns
/// BTreeMap mit Optionen für rustic_backend::OpenDALBackend
///
/// # Errors
/// Gibt einen Fehler zurück wenn:
/// - Provider nicht unterstützt wird
/// - Erforderliche Felder fehlen
///
/// # Example
/// ```no_run
/// use rustic_gui_lib::rustic::backends::opendal::{OpenDALConfig, create_opendal_backend};
/// use std::collections::BTreeMap;
///
/// let config = OpenDALConfig {
///     provider: "s3".to_string(),
///     endpoint: "my-bucket".to_string(),
///     access_key: "AKIAIOSFODNN7EXAMPLE".to_string(),
///     secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string(),
///     region: Some("us-east-1".to_string()),
///     endpoint_url: None,
///     extra_options: None,
/// };
///
/// let backend_options = create_opendal_backend(&config).unwrap();
/// ```
pub fn create_opendal_backend(config: &OpenDALConfig) -> Result<BTreeMap<String, String>> {
    let mut options = BTreeMap::new();

    match config.provider.as_str() {
        "s3" => {
            // AWS S3 oder S3-kompatible Services
            options.insert("bucket".to_string(), config.endpoint.clone());
            options.insert("access_key_id".to_string(), config.access_key.clone());
            options.insert(
                "secret_access_key".to_string(),
                config.secret_key.clone(),
            );

            if let Some(region) = &config.region {
                options.insert("region".to_string(), region.clone());
            }

            // Custom Endpoint für MinIO, Wasabi, etc.
            if let Some(endpoint_url) = &config.endpoint_url {
                options.insert("endpoint".to_string(), endpoint_url.clone());
            }
        }
        "azblob" => {
            // Azure Blob Storage
            options.insert("container".to_string(), config.endpoint.clone());
            options.insert("account_name".to_string(), config.access_key.clone());
            options.insert("account_key".to_string(), config.secret_key.clone());
        }
        "gcs" => {
            // Google Cloud Storage
            options.insert("bucket".to_string(), config.endpoint.clone());
            // GCS verwendet Service Account JSON als credential
            options.insert("credential".to_string(), config.secret_key.clone());
        }
        "b2" => {
            // Backblaze B2
            options.insert("bucket".to_string(), config.endpoint.clone());
            options.insert("application_key_id".to_string(), config.access_key.clone());
            options.insert(
                "application_key".to_string(),
                config.secret_key.clone(),
            );
        }
        _ => {
            return Err(crate::error::RusticGuiError::UnsupportedBackend {
                backend_type: config.provider.clone(),
            });
        }
    }

    // Extra options mergen
    if let Some(extra) = &config.extra_options {
        options.extend(extra.clone());
    }

    tracing::debug!(
        "OpenDAL-Backend erstellt: Provider={}, Endpoint={}",
        config.provider,
        config.endpoint
    );

    Ok(options)
}

/// Validiert eine OpenDAL-Konfiguration
///
/// # Arguments
/// * `config` - Zu validierende Konfiguration
///
/// # Returns
/// Ok(()) wenn Konfiguration valide ist
///
/// # Errors
/// Gibt einen Fehler zurück wenn erforderliche Felder fehlen
pub fn validate_opendal_config(config: &OpenDALConfig) -> Result<()> {
    if config.provider.is_empty() {
        return Err(crate::error::RusticGuiError::InvalidConfiguration {
            message: "Provider darf nicht leer sein".to_string(),
        });
    }

    if config.endpoint.is_empty() {
        return Err(crate::error::RusticGuiError::InvalidConfiguration {
            message: "Endpoint darf nicht leer sein".to_string(),
        });
    }

    if config.access_key.is_empty() {
        return Err(crate::error::RusticGuiError::InvalidConfiguration {
            message: "Access Key darf nicht leer sein".to_string(),
        });
    }

    if config.secret_key.is_empty() {
        return Err(crate::error::RusticGuiError::InvalidConfiguration {
            message: "Secret Key darf nicht leer sein".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_s3_backend() {
        let config = OpenDALConfig {
            provider: "s3".to_string(),
            endpoint: "my-test-bucket".to_string(),
            access_key: "test-access-key".to_string(),
            secret_key: "test-secret-key".to_string(),
            region: Some("us-west-2".to_string()),
            endpoint_url: None,
            extra_options: None,
        };

        let options = create_opendal_backend(&config).unwrap();

        assert_eq!(options.get("bucket"), Some(&"my-test-bucket".to_string()));
        assert_eq!(
            options.get("access_key_id"),
            Some(&"test-access-key".to_string())
        );
        assert_eq!(
            options.get("secret_access_key"),
            Some(&"test-secret-key".to_string())
        );
        assert_eq!(options.get("region"), Some(&"us-west-2".to_string()));
    }

    #[test]
    fn test_create_s3_backend_with_custom_endpoint() {
        let config = OpenDALConfig {
            provider: "s3".to_string(),
            endpoint: "my-bucket".to_string(),
            access_key: "minio-access".to_string(),
            secret_key: "minio-secret".to_string(),
            region: None,
            endpoint_url: Some("http://localhost:9000".to_string()),
            extra_options: None,
        };

        let options = create_opendal_backend(&config).unwrap();

        assert_eq!(
            options.get("endpoint"),
            Some(&"http://localhost:9000".to_string())
        );
    }

    #[test]
    fn test_create_azure_backend() {
        let config = OpenDALConfig {
            provider: "azblob".to_string(),
            endpoint: "my-container".to_string(),
            access_key: "storageaccountname".to_string(),
            secret_key: "storage-account-key".to_string(),
            region: None,
            endpoint_url: None,
            extra_options: None,
        };

        let options = create_opendal_backend(&config).unwrap();

        assert_eq!(options.get("container"), Some(&"my-container".to_string()));
        assert_eq!(
            options.get("account_name"),
            Some(&"storageaccountname".to_string())
        );
        assert_eq!(
            options.get("account_key"),
            Some(&"storage-account-key".to_string())
        );
    }

    #[test]
    fn test_create_gcs_backend() {
        let config = OpenDALConfig {
            provider: "gcs".to_string(),
            endpoint: "my-gcs-bucket".to_string(),
            access_key: "".to_string(), // Not used for GCS
            secret_key: r#"{"type":"service_account"}"#.to_string(),
            region: None,
            endpoint_url: None,
            extra_options: None,
        };

        let options = create_opendal_backend(&config).unwrap();

        assert_eq!(
            options.get("bucket"),
            Some(&"my-gcs-bucket".to_string())
        );
        assert!(options.get("credential").is_some());
    }

    #[test]
    fn test_unsupported_provider() {
        let config = OpenDALConfig {
            provider: "unsupported".to_string(),
            endpoint: "bucket".to_string(),
            access_key: "key".to_string(),
            secret_key: "secret".to_string(),
            region: None,
            endpoint_url: None,
            extra_options: None,
        };

        let result = create_opendal_backend(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_opendal_config_success() {
        let config = OpenDALConfig {
            provider: "s3".to_string(),
            endpoint: "bucket".to_string(),
            access_key: "key".to_string(),
            secret_key: "secret".to_string(),
            region: None,
            endpoint_url: None,
            extra_options: None,
        };

        assert!(validate_opendal_config(&config).is_ok());
    }

    #[test]
    fn test_validate_opendal_config_empty_provider() {
        let config = OpenDALConfig {
            provider: "".to_string(),
            endpoint: "bucket".to_string(),
            access_key: "key".to_string(),
            secret_key: "secret".to_string(),
            region: None,
            endpoint_url: None,
            extra_options: None,
        };

        assert!(validate_opendal_config(&config).is_err());
    }

    #[test]
    fn test_validate_opendal_config_empty_endpoint() {
        let config = OpenDALConfig {
            provider: "s3".to_string(),
            endpoint: "".to_string(),
            access_key: "key".to_string(),
            secret_key: "secret".to_string(),
            region: None,
            endpoint_url: None,
            extra_options: None,
        };

        assert!(validate_opendal_config(&config).is_err());
    }
}
