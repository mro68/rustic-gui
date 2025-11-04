use rustic_gui_lib::error::RusticGuiError;
use rustic_gui_lib::rustic::snapshot;
use rustic_gui_lib::types::RetentionPolicy;

#[tokio::test]
async fn test_list_snapshots_empty_repo() {
    // Arrange: Leeres Test-Repo (Mock oder temp dir, hier nur Dummy)
    let repo_path = "/tmp/nonexistent-repo";
    let password = "irrelevant";
    let result = snapshot::list_snapshots(repo_path, password).await;
    assert!(matches!(result, Err(RusticGuiError::RepositoryNotFound { .. })));
}

#[tokio::test]
async fn test_get_snapshot_not_found() {
    let repo_path = "/tmp/nonexistent-repo";
    let password = "irrelevant";
    let snap_id = "doesnotexist";
    let result = snapshot::get_snapshot(repo_path, password, snap_id).await;
    assert!(matches!(
        result,
        Err(RusticGuiError::RepositoryNotFound { .. })
            | Err(RusticGuiError::SnapshotNotFound { .. })
    ));
}

#[tokio::test]
async fn test_delete_snapshot_not_found() {
    let repo_path = "/tmp/nonexistent-repo";
    let password = "irrelevant";
    let snap_id = "doesnotexist";
    let result = snapshot::delete_snapshot(repo_path, password, snap_id).await;
    assert!(matches!(
        result,
        Err(RusticGuiError::RepositoryNotFound { .. }) | Err(RusticGuiError::Internal(_))
    ));
}

#[tokio::test]
async fn test_forget_snapshots_policy_on_empty_repo() {
    let repo_path = "/tmp/nonexistent-repo";
    let password = "irrelevant";
    let policy = RetentionPolicy::default();
    let result = snapshot::forget_snapshots(repo_path, password, &policy).await;
    assert!(matches!(
        result,
        Err(RusticGuiError::RepositoryNotFound { .. }) | Err(RusticGuiError::Internal(_))
    ));
}
