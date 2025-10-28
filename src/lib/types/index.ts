// TypeScript Interfaces für DTOs
// Diese entsprechen den Rust DTOs in src-tauri/src/types.rs

export interface RepositoryDto {
  id: string;
  name: string;
  path: string;
  repository_type: RepositoryType;
  status: RepositoryStatus;
  snapshot_count: number;
  total_size: number;
  last_accessed?: string; // ISO 8601
  created_at: string; // ISO 8601
}

export type RepositoryType = 'Local' | 'Sftp' | 'S3' | 'Rest' | 'Rclone';

export type RepositoryStatus = 'Healthy' | 'Warning' | 'Unavailable' | 'Locked';

export interface SnapshotDto {
  id: string;
  time: string; // ISO 8601
  hostname: string;
  tags: string[];
  paths: string[];
  file_count: number;
  total_size: number;
  repository_id: string;
  username?: string; // Optional: User who created the snapshot
  summary?: SnapshotSummary; // Optional: Additional summary info
}

export interface SnapshotSummary {
  files_count?: number;
  dirs_count?: number;
  data_size?: number;
}

export interface BackupJobDto {
  id: string;
  name: string;
  repository_id: string;
  source_paths: string[];
  tags: string[];
  schedule?: string; // Cron expression
  enabled: boolean;
  last_run?: string; // ISO 8601
  next_run?: string; // ISO 8601
  retention?: RetentionPolicy;
}

export interface RetentionPolicy {
  keep_last?: number;
  keep_daily?: number;
  keep_weekly?: number;
  keep_monthly?: number;
  keep_yearly?: number;
}

export interface RestoreOptionsDto {
  snapshot_id: string;
  target_path: string;
  paths: string[]; // Leer = alles wiederherstellen
  overwrite: boolean;
  restore_permissions: boolean;
  restore_timestamps: boolean;
  dry_run: boolean;
}

// Utility Types für API Responses
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  page_size: number;
}

// Progress Types für langlaufende Operationen
export interface ProgressInfo {
  current: number;
  total: number;
  message: string;
  percentage: number;
}

export interface BackupProgress extends ProgressInfo {
  files_processed: number;
  bytes_processed: number;
  current_file?: string;
  estimated_time_remaining?: number; // in Sekunden
}

export interface RestoreProgress extends ProgressInfo {
  files_restored: number;
  bytes_restored: number;
  current_file?: string;
}

// File Tree Types für Restore-Browser
export interface FileTreeNode {
  name: string;
  path: string;
  is_directory: boolean;
  size?: number;
  modified?: string; // ISO 8601
  children?: FileTreeNode[];
}
