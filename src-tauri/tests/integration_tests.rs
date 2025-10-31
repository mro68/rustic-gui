use rustic_gui_lib::{state::AppState, types::*};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_app_state_lifecycle() {
        // AppState erstellen
        let state = AppState::new().unwrap();

        // Initial sollte kein Repository geöffnet sein
        assert!(!state.has_current_repo());

        // Cancellation-Tokens sollten leer sein
        assert_eq!(state.cancellation_tokens.lock().len(), 0);
    }

    #[test]
    fn test_repository_dto_creation() {
        let dto = RepositoryDto {
            id: "test-repo".to_string(),
            name: "Test Repository".to_string(),
            path: "/tmp/test".to_string(),
            repository_type: RepositoryType::Local,
            status: RepositoryStatus::Healthy,
            snapshot_count: 5,
            total_size: 1024,
            last_accessed: Some("2025-10-26T10:00:00Z".to_string()),
            created_at: "2025-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(dto.id, "test-repo");
        assert_eq!(dto.name, "Test Repository");
        assert_eq!(dto.snapshot_count, 5);
        assert_eq!(dto.total_size, 1024);
        assert_eq!(dto.repository_type, RepositoryType::Local);
        assert_eq!(dto.status, RepositoryStatus::Healthy);
    }

    #[test]
    fn test_backup_job_dto_creation() {
        let job = BackupJobDto {
            id: "job-1".to_string(),
            name: "Daily Backup".to_string(),
            repository_id: "repo-1".to_string(),
            source_paths: vec!["/home/user".to_string()],
            tags: vec!["daily".to_string()],
            schedule: Some("0 2 * * *".to_string()),
            enabled: true,
            last_run: None,
            next_run: Some("2025-10-27T02:00:00Z".to_string()),
            retention: None,
        };

        assert_eq!(job.id, "job-1");
        assert_eq!(job.name, "Daily Backup");
        assert!(job.enabled);
        assert_eq!(job.source_paths.len(), 1);
        assert_eq!(job.tags.len(), 1);
    }

    #[test]
    fn test_snapshot_dto_creation() {
        let snapshot = SnapshotDto {
            id: "snapshot-1".to_string(),
            time: "2025-10-26T10:00:00Z".to_string(),
            hostname: "myhost".to_string(),
            tags: vec!["daily".to_string(), "important".to_string()],
            paths: vec!["/home/user".to_string()],
            file_count: 1337,
            total_size: 512 * 1024 * 1024, // 512MB
            repository_id: "repo-1".to_string(),
            username: Some("testuser".to_string()),
            summary: None,
        };

        assert_eq!(snapshot.id, "snapshot-1");
        assert_eq!(snapshot.hostname, "myhost");
        assert_eq!(snapshot.tags.len(), 2);
        assert_eq!(snapshot.file_count, 1337);
        assert_eq!(snapshot.total_size, 512 * 1024 * 1024);
    }

    #[test]
    fn test_restore_options_dto_creation() {
        let options = RestoreOptionsDto {
            snapshot_id: "snapshot-1".to_string(),
            target_path: "/tmp/restore".to_string(),
            paths: vec!["file1.txt".to_string(), "dir1".to_string()],
            overwrite: true,
            restore_permissions: true,
            restore_timestamps: false,
            dry_run: false,
        };

        assert_eq!(options.snapshot_id, "snapshot-1");
        assert_eq!(options.target_path, "/tmp/restore");
        assert_eq!(options.paths.len(), 2);
        assert!(options.overwrite);
        assert!(options.restore_permissions);
        assert!(!options.restore_timestamps);
        assert!(!options.dry_run);
    }

    #[test]
    fn test_repository_type_serialization() {
        // Teste dass alle RepositoryTypes korrekt serialisiert werden können
        let local = RepositoryType::Local;
        let json = serde_json::to_string(&local).unwrap();
        assert!(json.contains("Local"));
        
        let sftp = RepositoryType::Sftp;
        let json = serde_json::to_string(&sftp).unwrap();
        assert!(json.contains("Sftp"));
    }

    #[test]
    fn test_repository_status_serialization() {
        // Teste dass alle RepositoryStatus korrekt serialisiert werden können
        let healthy = RepositoryStatus::Healthy;
        let json = serde_json::to_string(&healthy).unwrap();
        assert!(json.contains("Healthy"));
        
        let warning = RepositoryStatus::Warning;
        let json = serde_json::to_string(&warning).unwrap();
        assert!(json.contains("Warning"));
    }

    #[test]
    fn test_retention_policy_creation() {
        let policy = RetentionPolicy {
            keep_last: Some(10),
            keep_daily: Some(7),
            keep_weekly: Some(4),
            keep_monthly: Some(12),
            keep_yearly: Some(2),
        };

        assert_eq!(policy.keep_last, Some(10));
        assert_eq!(policy.keep_daily, Some(7));
        assert_eq!(policy.keep_weekly, Some(4));
        assert_eq!(policy.keep_monthly, Some(12));
        assert_eq!(policy.keep_yearly, Some(2));
    }

    #[test]
    fn test_app_state_config_save_placeholder() {
        let state = AppState::new().unwrap();
        // Config-Save ist für jetzt ein Platzhalter
        let result = state.save_config();
        assert!(result.is_ok());
    }

    #[test]
    fn test_cancellation_tokens_management() {
        let state = AppState::new().unwrap();

        // Initial leer
        assert_eq!(state.cancellation_tokens.lock().len(), 0);

        // Token hinzufügen
        use tokio_util::sync::CancellationToken;
        let token = CancellationToken::new();
        state.cancellation_tokens.lock().insert("job-1".to_string(), token.clone());

        assert_eq!(state.cancellation_tokens.lock().len(), 1);

        // Token entfernen
        state.cancellation_tokens.lock().remove("job-1");

        assert_eq!(state.cancellation_tokens.lock().len(), 0);
    }
}
