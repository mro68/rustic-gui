<!-- GeneralTab.svelte: General Job Settings (Name, Repository, Tags) -->
<script lang="ts">
  import type { RepositoryDto } from '$lib/types';
  import CustomSelect from '../../shared/CustomSelect.svelte';

  interface GeneralTabProps {
    jobName?: string;
    selectedRepository?: string;
    jobTags?: string;
    savePassword?: boolean;
    jobPassword?: string;
    repositories: RepositoryDto[];
    errors?: Record<string, string>;
  }

  let {
    jobName = $bindable(''),
    selectedRepository = $bindable(''),
    jobTags = $bindable(''),
    savePassword = $bindable(false),
    jobPassword = $bindable(''),
    repositories = [],
    errors = {},
  }: GeneralTabProps = $props();
</script>

<div class="form-group">
  <label class="form-label" for="job-name">Job-Name</label>
  <input
    id="job-name"
    class="form-input"
    type="text"
    bind:value={jobName}
    placeholder="z.B. Dokumente täglich sichern"
  />
  {#if errors.jobName}<div class="error">{errors.jobName}</div>{/if}
</div>

<div class="form-group">
  <label class="form-label" for="repository">Ziel-Repository</label>
  <CustomSelect
    bind:value={selectedRepository}
    options={[
      { value: '', label: 'Repository auswählen...', disabled: true },
      ...repositories.map((repo) => ({
        value: repo.id,
        label: `${repo.name} - ${repo.path}`,
      })),
    ]}
    placeholder="Repository auswählen..."
  />
  {#if errors.repository}<div class="error">{errors.repository}</div>{/if}
</div>

<div class="form-group">
  <label class="form-label" for="tags">Tags</label>
  <input
    id="tags"
    class="form-input"
    type="text"
    bind:value={jobTags}
    placeholder="daily, documents, important"
  />
  <div class="form-help">Komma-getrennte Tags zur Organisation der Snapshots</div>
</div>

<div class="form-group">
  <div class="checkbox-group">
    <input type="checkbox" id="save-password" bind:checked={savePassword} />
    <label for="save-password">Passwort im Job speichern (verschlüsselt in Config)</label>
  </div>
  {#if savePassword}
    <div style="margin-top: 12px;">
      <label class="form-label" for="job-password">Repository-Passwort</label>
      <input
        id="job-password"
        class="form-input"
        type="password"
        bind:value={jobPassword}
        placeholder="Passwort eingeben..."
      />
      <div class="form-help">
        Wird verschlüsselt mit AES-256-GCM in der Config gespeichert. Andernfalls wird beim
        Backup-Start nach dem Passwort gefragt.
      </div>
    </div>
  {/if}
</div>

<style>
  .form-group {
    margin-bottom: 20px;
  }

  .form-label {
    display: block;
    font-size: 13px;
    font-weight: 500;
    color: #e4e4e7;
    margin-bottom: 8px;
  }

  .form-input,
  .form-select {
    width: 100%;
    padding: 10px 12px;
    background: #1a1d2e;
    border: 1px solid #2d3348;
    border-radius: 6px;
    color: #e4e4e7;
    font-size: 14px;
    font-family: inherit;
  }

  .form-input:focus,
  .form-select:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .checkbox-group {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 12px;
  }

  .checkbox-group input[type='checkbox'] {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }

  .checkbox-group label {
    font-size: 14px;
    color: #e4e4e7;
    cursor: pointer;
  }

  .form-help {
    font-size: 12px;
    color: #71717a;
    margin-top: 4px;
  }

  .error {
    color: #ef4444;
    font-size: 12px;
    margin-top: 4px;
  }
</style>
