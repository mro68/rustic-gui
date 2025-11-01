<!-- EditJobDialog.svelte: Dialog zum Bearbeiten von Backup-Jobs (gemäß Mockup) -->
<!--
  TODO.md: Phase 2 - Dialog-Workflow: Backup & Restore ✅ IMPLEMENTIERT
  Referenz: TODO.md Zeile 237-244, 332-340
  
  Status:
  - handleSubmit an api.updateBackupJob angebunden ✅ (TODO.md Zeile 239)
  
  Backend-Command: update_backup_job
  - src-tauri/src/commands/backup.rs:122
  
  API-Wrapper: src/lib/api/backup-jobs.ts:updateBackupJob
  
  Verwendung:
  - src/lib/components/pages/BackupJobs.svelte
-->
<script lang="ts">
  /**
   * Dialog zum Bearbeiten bestehender Backup-Jobs.
   *
   * Ermöglicht Änderungen an Name, Schedule, Paths, Retention-Policies.
   *
   * @component
   *
   * @example
   * ```svelte
   * <EditJobDialog
   *   bind:open={showEdit}
   *   {job}
   *   {repositories}
   *   on:saved={handleJobUpdated}
   * />
   * ```
   */
  import Button from '$lib/components/shared/Button.svelte';
  import Checkbox from '$lib/components/shared/Checkbox.svelte';
  import Input from '$lib/components/shared/Input.svelte';
  import Modal from '$lib/components/shared/Modal.svelte';
  import Select from '$lib/components/shared/Select.svelte';
  import { toastStore } from '$lib/stores/toast';
  import type { BackupJobDto, RepositoryDto } from '$lib/types';
  import { createEventDispatcher } from 'svelte';

  interface EditJobDialogProps {
    /** Steuert Sichtbarkeit */
    open?: boolean;
    /** Zu bearbeitender Job */
    job?: BackupJobDto | null;
    /** Verfügbare Repositories */
    repositories?: RepositoryDto[];
  }

  let { open = $bindable(false), job = null, repositories = [] }: EditJobDialogProps = $props();

  const dispatch = createEventDispatcher();

  // Form State
  let currentTab = $state(0);
  let isSubmitting = $state(false);

  // General Tab
  let jobName = $state('');
  let selectedRepositoryId = $state('');
  let tags = $state('');

  // Paths & Exclusions Tab
  let backupPaths = $state('');
  let excludePatterns = $state('');

  // Schedule Tab
  let scheduleType = $state('manual'); // manual, daily, weekly, monthly, cron
  let scheduleTime = $state('02:00');
  let scheduleDays = $state([] as string[]);
  let cronExpression = $state('');

  // Retention Tab
  let keepLast = $state(10);
  let keepHourly = $state(24);
  let keepDaily = $state(7);
  let keepWeekly = $state(4);
  let keepMonthly = $state(6);
  let keepYearly = $state(1);

  // Schedule Presets
  const schedulePresets = [
    { value: 'manual', label: 'Manuell' },
    { value: 'daily', label: 'Täglich' },
    { value: 'weekly', label: 'Wöchentlich' },
    { value: 'monthly', label: 'Monatlich' },
    { value: 'cron', label: 'Cron-Ausdruck' },
  ];

  const weekdays = [
    { value: '1', label: 'Montag' },
    { value: '2', label: 'Dienstag' },
    { value: '3', label: 'Mittwoch' },
    { value: '4', label: 'Donnerstag' },
    { value: '5', label: 'Freitag' },
    { value: '6', label: 'Samstag' },
    { value: '0', label: 'Sonntag' },
  ];

  // Initialize form when job changes
  $effect(() => {
    if (job && open) {
      initializeForm();
    }
  });

  function initializeForm() {
    if (!job) return;

    // General
    jobName = job.name;
    selectedRepositoryId = job.repository_id;
    tags = job.tags?.join(', ') || '';

    // Paths & Exclusions
    backupPaths = job.source_paths?.join('\n') || '';
    excludePatterns = job.exclude_patterns?.join('\n') || '';

    // Schedule - ist ein Cron-String, nicht ein Object
    cronExpression = job.schedule || '';
    scheduleType = job.schedule ? 'cron' : 'manual';
    scheduleTime = '02:00';
    scheduleDays = [];

    // Retention
    keepLast = job.retention?.keep_last || 10;
    keepHourly = 24; // keep_hourly existiert nicht im DTO
    keepDaily = job.retention?.keep_daily || 7;
    keepWeekly = job.retention?.keep_weekly || 4;
    keepMonthly = job.retention?.keep_monthly || 6;
    keepYearly = job.retention?.keep_yearly || 1;
  }

  function resetForm() {
    currentTab = 0;
    jobName = '';
    selectedRepositoryId = '';
    tags = '';
    backupPaths = '';
    excludePatterns = '';
    scheduleType = 'manual';
    scheduleTime = '02:00';
    scheduleDays = [];
    cronExpression = '';
    keepLast = 10;
    keepHourly = 24;
    keepDaily = 7;
    keepWeekly = 4;
    keepMonthly = 6;
    keepYearly = 1;
  }

  function validateCurrentTab(): boolean {
    switch (currentTab) {
      case 0: // General
        if (!jobName.trim()) {
          toastStore.error('Job-Name ist erforderlich');
          return false;
        }
        if (!selectedRepositoryId) {
          toastStore.error('Repository muss ausgewählt werden');
          return false;
        }
        return true;

      case 1: // Paths & Exclusions
        if (!backupPaths.trim()) {
          toastStore.error('Mindestens ein Backup-Pfad ist erforderlich');
          return false;
        }
        return true;

      case 2: // Schedule
        if (scheduleType === 'cron' && !cronExpression.trim()) {
          toastStore.error('Cron-Ausdruck ist erforderlich');
          return false;
        }
        if (scheduleType === 'weekly' && scheduleDays.length === 0) {
          toastStore.error('Mindestens ein Wochentag muss ausgewählt werden');
          return false;
        }
        return true;

      case 3: // Retention
        return true;

      default:
        return true;
    }
  }

  function nextTab() {
    if (!validateCurrentTab()) return;
    if (currentTab < 3) currentTab++;
  }

  function prevTab() {
    if (currentTab > 0) currentTab--;
  }

  async function handleSubmit() {
    if (!validateCurrentTab()) return;

    isSubmitting = true;
    try {
      const updatedJob = {
        ...job,
        id: job?.id || '',
        name: jobName.trim(),
        repository_id: selectedRepositoryId,
        tags: tags
          .split(',')
          .map((t) => t.trim())
          .filter((t) => t),
        source_paths: backupPaths
          .split('\n')
          .map((p) => p.trim())
          .filter((p) => p),
        exclude_patterns: excludePatterns
          .split('\n')
          .map((p) => p.trim())
          .filter((p) => p).length > 0 ? excludePatterns
            .split('\n')
            .map((p) => p.trim())
            .filter((p) => p) : undefined,
        schedule:
          scheduleType === 'manual'
            ? undefined
            : cronExpression || undefined,
        enabled: true,
        retention: {
          keep_last: keepLast > 0 ? keepLast : undefined,
          keep_daily: keepDaily > 0 ? keepDaily : undefined,
          keep_weekly: keepWeekly > 0 ? keepWeekly : undefined,
          keep_monthly: keepMonthly > 0 ? keepMonthly : undefined,
          keep_yearly: keepYearly > 0 ? keepYearly : undefined,
        },
      };

      try {
        const { updateBackupJob } = await import('$lib/api/backup-jobs');

        if (!job) {
          throw new Error('Kein Job zum Aktualisieren vorhanden');
        }

        // Konvertiere Schedule in Cron-Expression
        let cronSchedule: string | undefined;
        if (scheduleType !== 'manual') {
          if (scheduleType === 'custom' && cronExpression) {
            cronSchedule = cronExpression;
          } else if (scheduleTime) {
            const [hours, minutes] = scheduleTime.split(':');
            switch (scheduleType) {
              case 'daily':
                cronSchedule = `${minutes} ${hours} * * *`;
                break;
              case 'weekly':
                cronSchedule = `${minutes} ${hours} * * ${scheduleDays.join(',')}`;
                break;
              case 'monthly':
                cronSchedule = `${minutes} ${hours} 1 * *`;
                break;
            }
          }
        }

        await updateBackupJob({
          id: job.id,
          name: jobName,
          repository_id: selectedRepositoryId,
          source_paths: backupPaths
            .split('\n')
            .map((p) => p.trim())
            .filter((p) => p),
          exclude_patterns: excludePatterns
            .split('\n')
            .map((p) => p.trim())
            .filter((p) => p),
          tags: tags
            .split(',')
            .map((t) => t.trim())
            .filter((t) => t),
          schedule: cronSchedule,
          enabled: true,
          retention: {
            keep_last: keepLast,
            keep_daily: keepDaily,
            keep_weekly: keepWeekly,
            keep_monthly: keepMonthly,
            keep_yearly: keepYearly,
          },
        });

        console.log('Job updated successfully');
        dispatch('updated', updatedJob);
        open = false;
        resetForm();
        toastStore.success('Backup-Job wurde erfolgreich aktualisiert');
      } catch (error) {
        console.error('Failed to update job:', error);
        toastStore.error('Fehler beim Aktualisieren des Backup-Jobs');
      }
    } catch (error) {
      console.error('Failed to update job:', error);
      toastStore.error('Fehler beim Aktualisieren des Backup-Jobs');
    } finally {
      isSubmitting = false;
    }
  }

  function handleCancel() {
    open = false;
    resetForm();
  }

  function handleWeekdayToggle(day: string) {
    if (scheduleDays.includes(day)) {
      scheduleDays = scheduleDays.filter((d) => d !== day);
    } else {
      scheduleDays = [...scheduleDays, day];
    }
  }
</script>

<Modal bind:open ariaLabel="Backup-Job bearbeiten">
  <div class="edit-job-dialog">
    <!-- Tab Navigation -->
    <div class="tab-navigation">
      <button class="tab-button" class:active={currentTab === 0} onclick={() => (currentTab = 0)}>
        Allgemein
      </button>
      <button class="tab-button" class:active={currentTab === 1} onclick={() => (currentTab = 1)}>
        Pfade & Ausschlüsse
      </button>
      <button class="tab-button" class:active={currentTab === 2} onclick={() => (currentTab = 2)}>
        Zeitplan
      </button>
      <button class="tab-button" class:active={currentTab === 3} onclick={() => (currentTab = 3)}>
        Aufbewahrung
      </button>
    </div>

    <!-- Tab Content -->
    <div class="tab-content">
      {#if currentTab === 0}
        <!-- General Tab -->
        <div class="tab-panel">
          <div class="form-group">
            <Input
              label="Job-Name"
              placeholder="z.B. Tägliches Backup"
              bind:value={jobName}
              required
            />
          </div>

          <div class="form-group">
            <Select label="Repository" bind:value={selectedRepositoryId} required>
              <option value="">Repository auswählen...</option>
              {#each repositories as repo (repo.id)}
                <option value={repo.id}>{repo.name}</option>
              {/each}
            </Select>
          </div>

          <div class="form-group">
            <Input
              label="Tags (optional, kommagetrennt)"
              placeholder="z.B. wichtig, system, dokumente"
              bind:value={tags}
            />
          </div>
        </div>
      {:else if currentTab === 1}
        <!-- Paths & Exclusions Tab -->
        <div class="tab-panel">
          <div class="form-group">
            <div class="form-label">Backup-Pfade</div>
            <textarea
              class="form-textarea"
              placeholder="Ein Pfad pro Zeile, z.B.:
/home/user/dokumente
/var/www
/home/user/bilder"
              bind:value={backupPaths}
              rows="6"
            ></textarea>
            <div class="form-help">Geben Sie die zu sichernden Pfade ein, einen pro Zeile.</div>
          </div>

          <div class="form-group">
            <div class="form-label">Ausschlussmuster (optional)</div>
            <textarea
              class="form-textarea"
              placeholder="Ein Muster pro Zeile, z.B.:
*.tmp
.cache/
node_modules/"
              bind:value={excludePatterns}
              rows="4"
            ></textarea>
            <div class="form-help">
              Dateien/Ordner, die von der Sicherung ausgeschlossen werden sollen.
            </div>
          </div>
        </div>
      {:else if currentTab === 2}
        <!-- Schedule Tab -->
        <div class="tab-panel">
          <div class="form-group">
            <Select label="Zeitplan-Typ" bind:value={scheduleType}>
              {#each schedulePresets as preset (preset.value)}
                <option value={preset.value}>{preset.label}</option>
              {/each}
            </Select>
          </div>

          {#if scheduleType !== 'manual'}
            <div class="form-group">
              <Input label="Uhrzeit" type="time" bind:value={scheduleTime} />
            </div>
          {/if}

          {#if scheduleType === 'weekly'}
            <fieldset class="form-group">
              <legend class="form-label">Wochentage</legend>
              <div class="weekday-selector">
                {#each weekdays as day (day.value)}
                  <Checkbox
                    label={day.label}
                    checked={scheduleDays.includes(day.value)}
                    onchange={() => handleWeekdayToggle(day.value)}
                  />
                {/each}
              </div>
            </fieldset>
          {/if}

          {#if scheduleType === 'cron'}
            <div class="form-group">
              <Input
                label="Cron-Ausdruck"
                placeholder="z.B. 0 2 * * 1-5 (Mo-Fr 02:00)"
                bind:value={cronExpression}
              />
              <div class="form-help">Cron-Format: Minute Stunde Tag Monat Wochentag</div>
            </div>
          {/if}
        </div>
      {:else if currentTab === 3}
        <!-- Retention Tab -->
        <div class="tab-panel">
          <div class="form-description">
            Konfigurieren Sie, wie viele Snapshots aufbewahrt werden sollen.
          </div>

          <div class="retention-grid">
            <div class="form-group">
              <Input
                label="Letzte N Snapshots behalten"
                type="number"
                min="1"
                bind:value={keepLast}
              />
            </div>

            <div class="form-group">
              <Input
                label="Stündliche Snapshots (Stunden)"
                type="number"
                min="0"
                bind:value={keepHourly}
              />
            </div>

            <div class="form-group">
              <Input
                label="Tägliche Snapshots (Tage)"
                type="number"
                min="0"
                bind:value={keepDaily}
              />
            </div>

            <div class="form-group">
              <Input
                label="Wöchentliche Snapshots (Wochen)"
                type="number"
                min="0"
                bind:value={keepWeekly}
              />
            </div>

            <div class="form-group">
              <Input
                label="Monatliche Snapshots (Monate)"
                type="number"
                min="0"
                bind:value={keepMonthly}
              />
            </div>

            <div class="form-group">
              <Input
                label="Jährliche Snapshots (Jahre)"
                type="number"
                min="0"
                bind:value={keepYearly}
              />
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Dialog Actions -->
    <div class="dialog-actions">
      <div class="left-actions">
        {#if currentTab > 0}
          <Button variant="secondary" onclick={prevTab}>Zurück</Button>
        {/if}
      </div>

      <div class="right-actions">
        <Button variant="secondary" onclick={handleCancel}>Abbrechen</Button>

        {#if currentTab < 3}
          <Button variant="primary" onclick={nextTab}>Weiter</Button>
        {:else}
          <Button variant="primary" onclick={handleSubmit} disabled={isSubmitting}>
            {#if isSubmitting}
              Speichere...
            {:else}
              Speichern
            {/if}
          </Button>
        {/if}
      </div>
    </div>
  </div>
</Modal>

<style>
  .edit-job-dialog {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .tab-navigation {
    display: flex;
    border-bottom: 1px solid var(--border-color);
  }

  .tab-button {
    padding: 0.75rem 1rem;
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    border-bottom: 2px solid transparent;
    transition: all 0.2s ease;
  }

  .tab-button:hover {
    color: var(--text-primary);
  }

  .tab-button.active {
    color: var(--color-primary);
    border-bottom-color: var(--color-primary);
  }

  .tab-content {
    min-height: 400px;
  }

  .tab-panel {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-primary);
  }

  .form-textarea {
    padding: 0.75rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 0.375rem;
    color: var(--text-primary);
    font-family: monospace;
    font-size: 0.875rem;
    resize: vertical;
  }

  .form-textarea:focus {
    outline: none;
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
  }

  .form-help {
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .weekday-selector {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 0.75rem;
  }

  .form-description {
    font-size: 0.875rem;
    color: var(--text-secondary);
    margin-bottom: 1rem;
  }

  .retention-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .dialog-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 1.5rem;
    border-top: 1px solid var(--border-color);
  }

  .left-actions,
  .right-actions {
    display: flex;
    gap: 0.75rem;
  }

  @media (max-width: 768px) {
    .tab-navigation {
      flex-wrap: wrap;
    }

    .tab-button {
      flex: 1;
      min-width: 120px;
    }

    .retention-grid {
      grid-template-columns: 1fr;
    }

    .weekday-selector {
      grid-template-columns: repeat(2, 1fr);
    }

    .dialog-actions {
      flex-direction: column;
      gap: 1rem;
    }

    .left-actions,
    .right-actions {
      width: 100%;
      justify-content: center;
    }
  }
</style>
