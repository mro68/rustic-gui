<!-- JobDialog.svelte: Unified Dialog for Creating and Editing Backup Jobs -->
<!--
  Kombiniert CreateJobDialog und EditJobDialog in einen 4-Tab-Wizard.
  
  Mode:
  - create: Neuen Job erstellen
  - edit: Bestehenden Job bearbeiten
  
  Sub-Components:
  - GeneralTab: Name, Repository, Tags, Password
  - PathsTab: Source Paths & Exclusions
  - ScheduleTab: Cron Schedule Builder
  - RetentionTab: Retention Policy
-->
<script lang="ts">
  import { updateBackupJob } from '$lib/api/backup-jobs';
  import Button from '$lib/components/shared/Button.svelte';
  import Modal from '$lib/components/shared/Modal.svelte';
  import { toastStore } from '$lib/stores/toast';
  import type { BackupJobDto, RepositoryDto } from '$lib/types';
  import { createEventDispatcher } from 'svelte';
  import GeneralTab from './JobDialog/GeneralTab.svelte';
  import PathsTab from './JobDialog/PathsTab.svelte';
  import RetentionTab from './JobDialog/RetentionTab.svelte';
  import ScheduleTab from './JobDialog/ScheduleTab.svelte';

  interface JobDialogProps {
    /** Steuert Sichtbarkeit */
    open?: boolean;
    /** Modus: 'create' oder 'edit' */
    mode: 'create' | 'edit';
    /** Zu bearbeitender Job (nur bei mode='edit') */
    job?: BackupJobDto | null;
    /** Verfügbare Repositories */
    repositories?: RepositoryDto[];
  }

  let { open = $bindable(false), mode, job = null, repositories = [] }: JobDialogProps = $props();

  const dispatch = createEventDispatcher();

  // Tab State
  let currentTab = $state(0);
  const tabs = ['General', 'Paths & Exclusions', 'Schedule', 'Retention'];

  // Form State (all tabs)
  let jobName = $state('');
  let selectedRepository = $state('');
  let jobTags = $state('');
  let savePassword = $state(false);
  let jobPassword = $state('');
  let sourcePaths = $state<string[]>(['']);
  let excludePatterns = $state<string[]>(['']);
  let oneFileSystem = $state(true);
  let scheduleType = $state('daily');
  let scheduleTime = $state('02:00');
  let scheduleTimezone = $state('Local');
  let keepDaily = $state(7);
  let keepWeekly = $state(4);
  let keepMonthly = $state(6);
  let keepYearly = $state(2);
  let autoPrune = $state(true);

  // Validation
  let errors: Record<string, string> = $state({});
  let isSubmitting = $state(false);

  // Initialize form when job changes (edit mode)
  $effect(() => {
    if (job && open && mode === 'edit') {
      initializeForm();
    }
  });

  function initializeForm() {
    if (!job) return;

    jobName = job.name;
    selectedRepository = job.repository_id;
    jobTags = job.tags?.join(', ') || '';
    sourcePaths = job.source_paths?.length ? job.source_paths : [''];
    excludePatterns = job.exclude_patterns?.length ? job.exclude_patterns : [''];

    // Parse schedule (Cron -> Type)
    if (job.schedule) {
      scheduleType = 'custom'; // Default to custom if cron exists
      // Versuche simple Patterns zu erkennen
      const [min, hour, , , dow] = job.schedule.split(' ');
      scheduleTime = `${hour.padStart(2, '0')}:${min.padStart(2, '0')}`;

      if (job.schedule.match(/^\d+ \d+ \* \* \*$/)) scheduleType = 'daily';
      else if (job.schedule.match(/^\d+ \d+ \* \* 0$/)) scheduleType = 'weekly';
      else if (job.schedule.match(/^\d+ \d+ 1 \* \*$/)) scheduleType = 'monthly';
    }

    // Retention
    keepDaily = job.retention?.keep_daily || 7;
    keepWeekly = job.retention?.keep_weekly || 4;
    keepMonthly = job.retention?.keep_monthly || 6;
    keepYearly = job.retention?.keep_yearly || 2;
  }

  function validateCurrentTab(): boolean {
    errors = {};

    switch (currentTab) {
      case 0: // General
        if (!jobName.trim()) errors.jobName = 'Job-Name ist erforderlich';
        if (!selectedRepository) errors.repository = 'Repository ist erforderlich';
        break;
      case 1: // Paths
        if (sourcePaths.filter((p) => p.trim()).length === 0) {
          errors.sourcePaths = 'Mindestens ein Source-Pfad ist erforderlich';
        }
        break;
      case 2: // Schedule
        if (!scheduleTime) errors.scheduleTime = 'Zeit ist erforderlich';
        break;
      case 3: // Retention
        if (keepDaily < 0 || keepWeekly < 0 || keepMonthly < 0 || keepYearly < 0) {
          errors.retention = 'Retention-Werte müssen >= 0 sein';
        }
        break;
    }

    return Object.keys(errors).length === 0;
  }

  function nextTab() {
    if (validateCurrentTab()) {
      currentTab = Math.min(currentTab + 1, tabs.length - 1);
    }
  }

  function prevTab() {
    currentTab = Math.max(currentTab - 1, 0);
  }

  function resetForm() {
    jobName = '';
    selectedRepository = '';
    jobTags = '';
    savePassword = false;
    jobPassword = '';
    sourcePaths = [''];
    excludePatterns = [''];
    oneFileSystem = true;
    scheduleType = 'daily';
    scheduleTime = '02:00';
    scheduleTimezone = 'Local';
    keepDaily = 7;
    keepWeekly = 4;
    keepMonthly = 6;
    keepYearly = 2;
    autoPrune = true;
    currentTab = 0;
    errors = {};
  }

  function close() {
    open = false;
    resetForm();
  }

  /**
   * Konvertiert Schedule-Type + Time in Cron-Expression
   */
  function buildCronExpression(): string | undefined {
    if (scheduleType === 'manual') return undefined;

    const [hours, minutes] = scheduleTime.split(':');
    switch (scheduleType) {
      case 'daily':
        return `${minutes} ${hours} * * *`;
      case 'weekly':
        return `${minutes} ${hours} * * 0`;
      case 'monthly':
        return `${minutes} ${hours} 1 * *`;
      default:
        return undefined;
    }
  }

  async function handleSubmit() {
    if (!validateCurrentTab()) return;

    isSubmitting = true;
    errors = {};

    try {
      const cronSchedule = buildCronExpression();

      const jobData = {
        name: jobName,
        repository_id: selectedRepository,
        source_paths: sourcePaths.filter((p) => p.trim()),
        exclude_patterns: excludePatterns.filter((p) => p.trim()),
        tags: jobTags
          .split(',')
          .map((t) => t.trim())
          .filter((t) => t),
        schedule: cronSchedule,
        retention: {
          keep_daily: keepDaily,
          keep_weekly: keepWeekly,
          keep_monthly: keepMonthly,
          keep_yearly: keepYearly,
        },
        password: savePassword ? jobPassword : undefined,
      };

      if (mode === 'create') {
        // CREATE
        const { createBackupJob, scheduleBackup } = await import('$lib/api/backup-jobs');
        const jobId = await createBackupJob(jobData);

        // Auto-schedule if cron given
        if (cronSchedule && scheduleType !== 'manual') {
          try {
            await scheduleBackup(jobId, cronSchedule);
            console.log(`Job ${jobId} geplant mit Cron: ${cronSchedule}`);
          } catch (scheduleError) {
            console.error('Failed to schedule job:', scheduleError);
            errors.general = `Job erstellt, aber Planung fehlgeschlagen: ${scheduleError}`;
          }
        }

        toastStore.success('Backup-Job erfolgreich erstellt');
        dispatch('created', { ...jobData, id: jobId });
      } else {
        // EDIT
        if (!job?.id) throw new Error('Job-ID fehlt');

        await updateBackupJob({ ...jobData, id: job.id } as BackupJobDto);
        toastStore.success('Backup-Job erfolgreich aktualisiert');
        dispatch('saved', { ...jobData, id: job.id });
      }

      close();
    } catch (error) {
      console.error(`Failed to ${mode} job:`, error);
      errors.general = `Fehler beim ${mode === 'create' ? 'Erstellen' : 'Aktualisieren'} des Jobs: ${error}`;
      toastStore.error(errors.general);
    } finally {
      isSubmitting = false;
    }
  }

  // Derived states
  const dialogTitle = $derived(
    mode === 'create' ? 'Backup-Job erstellen' : 'Backup-Job bearbeiten'
  );
  const submitLabel = $derived(mode === 'create' ? 'Job erstellen' : 'Änderungen speichern');
</script>

<Modal bind:open ariaLabel={dialogTitle}>
  {#snippet header()}
    <h2>{dialogTitle}</h2>
  {/snippet}

  <!-- Tabs -->
  <div class="tabs">
    {#each tabs as tab, index}
      <button
        class="tab {currentTab === index ? 'active' : ''}"
        onclick={() => {
          if (validateCurrentTab() || index < currentTab) currentTab = index;
        }}
      >
        {tab}
      </button>
    {/each}
  </div>

  <div class="tab-content">
    {#if currentTab === 0}
      <GeneralTab
        bind:jobName
        bind:selectedRepository
        bind:jobTags
        bind:savePassword
        bind:jobPassword
        {repositories}
        {errors}
      />
    {:else if currentTab === 1}
      <PathsTab bind:sourcePaths bind:excludePatterns bind:oneFileSystem {errors} />
    {:else if currentTab === 2}
      <ScheduleTab bind:scheduleType bind:scheduleTime bind:scheduleTimezone {errors} />
    {:else if currentTab === 3}
      <RetentionTab
        bind:keepDaily
        bind:keepWeekly
        bind:keepMonthly
        bind:keepYearly
        bind:autoPrune
        {errors}
      />
    {/if}

    {#if errors.general}
      <div class="error-box">{errors.general}</div>
    {/if}
  </div>

  {#snippet footer()}
    {#if currentTab > 0}
      <Button variant="secondary" onclick={prevTab} disabled={isSubmitting}>Zurück</Button>
    {/if}
    {#if currentTab < tabs.length - 1}
      <Button variant="primary" onclick={nextTab} disabled={isSubmitting}>Weiter</Button>
    {:else}
      <Button variant="primary" onclick={handleSubmit} disabled={isSubmitting}>
        {isSubmitting ? 'Wird gespeichert...' : submitLabel}
      </Button>
    {/if}
  {/snippet}
</Modal>

<style>
  .tabs {
    display: flex;
    gap: 4px;
    border-bottom: 1px solid #2d3348;
    margin-bottom: 24px;
  }

  .tab {
    padding: 10px 16px;
    cursor: pointer;
    color: #71717a;
    font-size: 14px;
    border-bottom: 2px solid transparent;
    transition: all 0.2s;
  }

  .tab:hover {
    color: #e4e4e7;
  }

  .tab.active {
    color: #e4e4e7;
    border-bottom-color: #3b82f6;
  }

  .tab-content {
    min-height: 300px;
  }

  .error-box {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #ef4444;
    padding: 12px 16px;
    border-radius: 8px;
    margin-top: 16px;
    font-size: 14px;
  }
</style>
