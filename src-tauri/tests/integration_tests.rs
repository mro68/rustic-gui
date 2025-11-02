use rustic_gui_lib::{state::AppState, types::*};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_check_result_dto() {
        let dto = CheckResultDto {
            errors: vec!["Error 1".to_string()],
            warnings: vec!["Warning 1".to_string()],
            is_ok: false,
        };

        assert_eq!(dto.errors.len(), 1);
        assert_eq!(dto.warnings.len(), 1);
        assert!(!dto.is_ok);
    }

    #[test]
    fn test_prune_result_dto() {
        let dto = PruneResultDto {
            packs_removed: 10,
            packs_kept: 50,
            packs_recovered: 2,
            size_removed: 1024 * 1024,
            size_kept: 512 * 1024 * 1024,
            size_recovered: 1024,
            dry_run: false,
        };

        assert_eq!(dto.packs_removed, 10);
        assert_eq!(dto.size_removed, 1024 * 1024);
        assert!(!dto.dry_run);
    }

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
            exclude_patterns: Some(vec!["*.tmp".to_string(), "node_modules".to_string()]),
            tags: vec!["daily".to_string()],
            schedule: Some("0 2 * * *".to_string()),
            enabled: true,
            last_run: None,
            next_run: Some("2025-10-27T02:00:00Z".to_string()),
            retention: None,
            password: None,
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
    }

    #[test]
    fn test_backup_progress_dto() {
        let progress = ProgressInfo {
            current: 50,
            total: 100,
            message: Some("Processing files...".to_string()),
            percentage: Some(50.0),
        };

        assert_eq!(progress.current, 50);
        assert_eq!(progress.total, 100);
        assert_eq!(progress.percentage, Some(50.0));
    }

    #[test]
    fn test_restore_progress_dto() {
        let base = ProgressInfo {
            current: 10,
            total: 50,
            message: Some("Restoring...".to_string()),
            percentage: Some(20.0),
        };

        let progress = RestoreProgress {
            base,
            files_restored: 10,
            bytes_restored: 1024 * 512,
            current_file: Some("/home/user/file.txt".to_string()),
        };

        assert_eq!(progress.files_restored, 10);
        assert_eq!(progress.bytes_restored, 1024 * 512);
        assert!(progress.current_file.is_some());
    }

    #[test]
    fn test_backup_workflow_types() {
        // Test kompletter Backup-Workflow mit DTOs

        // 1. Repository erstellen
        let repo = RepositoryDto {
            id: "repo-1".to_string(),
            name: "Test Repo".to_string(),
            path: "/tmp/test-repo".to_string(),
            repository_type: RepositoryType::Local,
            status: RepositoryStatus::Healthy,
            snapshot_count: 0,
            total_size: 0,
            last_accessed: None,
            created_at: "2025-11-02T00:00:00Z".to_string(),
        };
        assert_eq!(repo.snapshot_count, 0);

        // 2. Backup-Job erstellen
        let job = BackupJobDto {
            id: "job-1".to_string(),
            name: "Test Backup".to_string(),
            repository_id: repo.id.clone(),
            source_paths: vec!["/tmp/source".to_string()],
            exclude_patterns: Some(vec!["*.tmp".to_string()]),
            tags: vec!["test".to_string()],
            schedule: None,
            enabled: true,
            last_run: None,
            next_run: None,
            retention: Some(RetentionPolicy {
                keep_last: Some(5),
                keep_daily: Some(7),
                keep_weekly: None,
                keep_monthly: None,
                keep_yearly: None,
            }),
            password: None,
        };
        assert_eq!(job.repository_id, repo.id);

        // 3. Snapshot nach Backup
        let snapshot = SnapshotDto {
            id: "snap-1".to_string(),
            time: "2025-11-02T10:00:00Z".to_string(),
            hostname: "testhost".to_string(),
            tags: job.tags.clone(),
            paths: job.source_paths.clone(),
            file_count: 100,
            total_size: 1024 * 1024,
            repository_id: repo.id.clone(),
            username: Some("testuser".to_string()),
            summary: None,
        };
        assert_eq!(snapshot.repository_id, repo.id);
        assert_eq!(snapshot.tags, job.tags);

        // 4. Restore-Options
        let restore_opts = RestoreOptionsDto {
            snapshot_id: snapshot.id.clone(),
            target_path: "/tmp/restore".to_string(),
            paths: vec![],
            overwrite: true,
            restore_permissions: true,
            restore_timestamps: true,
            dry_run: false,
        };
        assert_eq!(restore_opts.snapshot_id, snapshot.id);
    }

    #[test]
    fn test_repository_maintenance_workflow() {
        // Test Repository-Wartung Workflow mit DTOs

        // 1. Check Result
        let check_result = CheckResultDto { errors: vec![], warnings: vec![], is_ok: true };
        assert!(check_result.is_ok);
        assert_eq!(check_result.errors.len(), 0);

        // 2. Prune Result (nach Snapshot-Löschungen)
        let prune_result = PruneResultDto {
            packs_removed: 5,
            packs_kept: 45,
            packs_recovered: 0,
            size_removed: 10 * 1024 * 1024,
            size_kept: 500 * 1024 * 1024,
            size_recovered: 0,
            dry_run: false,
        };
        assert_eq!(prune_result.packs_removed, 5);
        assert!(!prune_result.dry_run);
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
