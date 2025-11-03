<!-- RetentionTab.svelte: Retention Policy Configuration -->
<script lang="ts">
  import CustomSelect from '../../shared/CustomSelect.svelte';

  interface RetentionTabProps {
    keepDaily?: number;
    keepWeekly?: number;
    keepMonthly?: number;
    keepYearly?: number;
    autoPrune?: boolean;
    errors?: Record<string, string>;
  }

  let {
    keepDaily = $bindable(7),
    keepWeekly = $bindable(4),
    keepMonthly = $bindable(6),
    keepYearly = $bindable(2),
    autoPrune = $bindable(true),
    errors = {},
  }: RetentionTabProps = $props();

  let retentionType = $state('advanced');

  const retentionSummary = $derived(
    `Täglich: ${keepDaily}, Wöchentlich: ${keepWeekly}, Monatlich: ${keepMonthly}, Jährlich: ${keepYearly}`
  );
</script>

<div class="form-group">
  <div class="form-label">Aufbewahrungs-Richtlinie</div>
  <CustomSelect
    bind:value={retentionType}
    options={[
      { value: 'simple', label: 'Simple - Letzte N Snapshots behalten' },
      { value: 'advanced', label: 'Erweitert - Tägliche/Wöchentliche/Monatliche behalten' },
      { value: 'custom', label: 'Benutzerdefinierte forget-Richtlinie' },
    ]}
  />
</div>

<div class="form-row">
  <div class="form-group">
    <label class="form-label" for="keep-daily">Tägliche behalten</label>
    <input id="keep-daily" class="form-input" type="number" bind:value={keepDaily} min="0" />
  </div>
  <div class="form-group">
    <label class="form-label" for="keep-weekly">Wöchentliche behalten</label>
    <input id="keep-weekly" class="form-input" type="number" bind:value={keepWeekly} min="0" />
  </div>
</div>

<div class="form-row">
  <div class="form-group">
    <label class="form-label" for="keep-monthly">Monatliche behalten</label>
    <input id="keep-monthly" class="form-input" type="number" bind:value={keepMonthly} min="0" />
  </div>
  <div class="form-group">
    <label class="form-label" for="keep-yearly">Jährliche behalten</label>
    <input id="keep-yearly" class="form-input" type="number" bind:value={keepYearly} min="0" />
  </div>
</div>

<div class="info-box">📊 Richtlinie: {retentionSummary}</div>

<div class="checkbox-group">
  <input type="checkbox" id="auto-prune" bind:checked={autoPrune} />
  <label for="auto-prune">Automatisch alte Snapshots nach Backup entfernen</label>
</div>

{#if errors.retention}<div class="error">{errors.retention}</div>{/if}

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

  .form-input {
    width: 100%;
    padding: 10px 12px;
    background: #1a1d2e;
    border: 1px solid #2d3348;
    border-radius: 6px;
    color: #e4e4e7;
    font-size: 14px;
    font-family: inherit;
  }

  .form-input:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
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

  .info-box {
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.3);
    border-radius: 8px;
    padding: 12px 16px;
    font-size: 13px;
    color: #93c5fd;
    margin-bottom: 20px;
  }

  .error {
    color: #ef4444;
    font-size: 12px;
    margin-top: 4px;
  }

  @media (max-width: 768px) {
    .form-row {
      grid-template-columns: 1fr;
    }
  }
</style>
