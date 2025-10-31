/// Job-Scheduler für zeitgesteuerte Backups
///
/// Dieser Modul implementiert automatisierte Backup-Jobs mit Cron-Scheduling
/// basierend auf tokio-cron-scheduler.

use anyhow::Result;
use futures::future::BoxFuture;
use std::collections::HashMap;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

/// Backup-Scheduler für zeitgesteuerte Jobs
///
/// Verwaltet geplante Backup-Jobs und deren Ausführung.
pub struct BackupScheduler {
    /// Tokio-Cron-Scheduler-Instanz
    scheduler: JobScheduler,
    /// Mapping von Job-IDs zu Scheduler-UUIDs
    jobs: HashMap<String, Uuid>,
}

impl BackupScheduler {
    /// Erstellt einen neuen BackupScheduler
    ///
    /// # Returns
    /// Result mit BackupScheduler-Instanz
    ///
    /// # Errors
    /// Gibt einen Fehler zurück wenn Scheduler-Erstellung fehlschlägt
    pub async fn new() -> Result<Self> {
        let scheduler = JobScheduler::new().await?;
        scheduler.start().await?;

        tracing::info!("BackupScheduler erfolgreich initialisiert");

        Ok(Self {
            scheduler,
            jobs: HashMap::new(),
        })
    }

    /// Plant einen neuen Job
    ///
    /// # Arguments
    /// * `job_id` - Eindeutige Job-ID
    /// * `cron_expr` - Cron-Expression für Zeitplanung
    /// * `callback` - Async-Funktion die ausgeführt wird
    ///
    /// # Returns
    /// Result mit () bei Erfolg
    ///
    /// # Errors
    /// Gibt einen Fehler zurück wenn:
    /// - Cron-Expression ungültig ist
    /// - Job bereits existiert
    /// - Scheduler-Fehler auftritt
    pub async fn schedule_job<F>(
        &mut self,
        job_id: String,
        cron_expr: &str,
        callback: F,
    ) -> Result<()>
    where
        F: Fn() -> BoxFuture<'static, ()> + Send + Sync + 'static,
    {
        // Prüfe ob Job bereits existiert
        if self.jobs.contains_key(&job_id) {
            anyhow::bail!("Job mit ID '{}' existiert bereits", job_id);
        }

        // Erstelle Cron-Job
        let job = Job::new_async(cron_expr, move |_uuid, _lock| Box::pin(callback()))?;

        // Füge Job zum Scheduler hinzu
        let uuid = self.scheduler.add(job).await?;
        self.jobs.insert(job_id.clone(), uuid);

        tracing::info!("Job '{}' geplant mit Cron-Expression: {}", job_id, cron_expr);

        Ok(())
    }

    /// Entfernt einen geplanten Job
    ///
    /// # Arguments
    /// * `job_id` - ID des zu entfernenden Jobs
    ///
    /// # Returns
    /// Result mit () bei Erfolg
    ///
    /// # Errors
    /// Gibt einen Fehler zurück wenn Job nicht gefunden oder Entfernung fehlschlägt
    pub async fn remove_job(&mut self, job_id: &str) -> Result<()> {
        if let Some(uuid) = self.jobs.remove(job_id) {
            self.scheduler.remove(&uuid).await?;
            tracing::info!("Job '{}' entfernt", job_id);
            Ok(())
        } else {
            anyhow::bail!("Job mit ID '{}' nicht gefunden", job_id)
        }
    }

    /// Listet alle geplanten Job-IDs auf
    ///
    /// # Returns
    /// Vektor mit Job-IDs
    pub fn list_scheduled_jobs(&self) -> Vec<String> {
        self.jobs.keys().cloned().collect()
    }

    /// Prüft ob ein Job geplant ist
    ///
    /// # Arguments
    /// * `job_id` - Zu prüfende Job-ID
    ///
    /// # Returns
    /// true wenn Job existiert, sonst false
    pub fn has_job(&self, job_id: &str) -> bool {
        self.jobs.contains_key(job_id)
    }

    /// Stoppt den Scheduler
    ///
    /// # Returns
    /// Result mit () bei Erfolg
    pub async fn shutdown(&mut self) -> Result<()> {
        self.scheduler.shutdown().await?;
        tracing::info!("BackupScheduler gestoppt");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_scheduler_creation() {
        let scheduler = BackupScheduler::new().await;
        assert!(scheduler.is_ok());
    }

    #[tokio::test]
    async fn test_schedule_and_remove_job() {
        let mut scheduler = BackupScheduler::new().await.unwrap();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        // Schedule job (alle 2 Sekunden)
        let result = scheduler
            .schedule_job("test-job".to_string(), "*/2 * * * * *", move || {
                let counter = counter_clone.clone();
                Box::pin(async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                })
            })
            .await;

        assert!(result.is_ok());

        // Warte 5 Sekunden
        sleep(Duration::from_secs(5)).await;

        // Job sollte mindestens 2x ausgeführt worden sein
        let count = counter.load(Ordering::SeqCst);
        assert!(count >= 2, "Job wurde nur {}x ausgeführt", count);

        // Entferne Job
        let result = scheduler.remove_job("test-job").await;
        assert!(result.is_ok());

        // Liste sollte leer sein
        assert_eq!(scheduler.list_scheduled_jobs().len(), 0);
    }

    #[tokio::test]
    async fn test_list_scheduled_jobs() {
        let mut scheduler = BackupScheduler::new().await.unwrap();

        scheduler
            .schedule_job("job1".to_string(), "0 0 * * * *", || {
                Box::pin(async {})
            })
            .await
            .unwrap();

        scheduler
            .schedule_job("job2".to_string(), "0 0 * * * *", || {
                Box::pin(async {})
            })
            .await
            .unwrap();

        let jobs = scheduler.list_scheduled_jobs();
        assert_eq!(jobs.len(), 2);
        assert!(jobs.contains(&"job1".to_string()));
        assert!(jobs.contains(&"job2".to_string()));
    }

    #[tokio::test]
    async fn test_duplicate_job_fails() {
        let mut scheduler = BackupScheduler::new().await.unwrap();

        scheduler
            .schedule_job("duplicate".to_string(), "0 0 * * * *", || {
                Box::pin(async {})
            })
            .await
            .unwrap();

        let result = scheduler
            .schedule_job("duplicate".to_string(), "0 0 * * * *", || {
                Box::pin(async {})
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_cron_expression() {
        let mut scheduler = BackupScheduler::new().await.unwrap();

        let result = scheduler
            .schedule_job("invalid".to_string(), "invalid cron", || {
                Box::pin(async {})
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_has_job() {
        let mut scheduler = BackupScheduler::new().await.unwrap();

        assert!(!scheduler.has_job("test"));

        scheduler
            .schedule_job("test".to_string(), "0 0 * * * *", || {
                Box::pin(async {})
            })
            .await
            .unwrap();

        assert!(scheduler.has_job("test"));

        scheduler.remove_job("test").await.unwrap();

        assert!(!scheduler.has_job("test"));
    }
}
