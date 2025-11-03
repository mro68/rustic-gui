<!-- ScheduleTab.svelte: Job Schedule Configuration (Cron Builder) -->
<script lang="ts">
  import CustomSelect from '../../shared/CustomSelect.svelte';

  interface ScheduleTabProps {
    scheduleType?: string;
    scheduleTime?: string;
    scheduleTimezone?: string;
    errors?: Record<string, string>;
  }

  let {
    scheduleType = $bindable('daily'),
    scheduleTime = $bindable('02:00'),
    scheduleTimezone = $bindable('Local'),
    errors = {},
  }: ScheduleTabProps = $props();

  function selectScheduleType(type: string) {
    scheduleType = type;
  }

  const schedulePreview = $derived(
    (() => {
      switch (scheduleType) {
        case 'daily':
          return `Täglich um ${scheduleTime}`;
        case 'weekly':
          return `Wöchentlich (Sonntag) um ${scheduleTime}`;
        case 'monthly':
          return `Monatlich (1.) um ${scheduleTime}`;
        case 'custom':
          return `Benutzerdefiniert`;
        case 'manual':
          return `Manuell`;
        default:
          return '';
      }
    })()
  );
</script>

<div class="form-group">
  <div class="form-label">Zeitplan-Typ</div>
  <div class="schedule-builder">
    <div class="schedule-quick">
      <div
        class="schedule-preset {scheduleType === 'daily' ? 'selected' : ''}"
        onclick={() => selectScheduleType('daily')}
        onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && selectScheduleType('daily')}
        role="button"
        tabindex="0"
      >
        <div style="margin-bottom: 4px;">📅</div>
        <div>Täglich</div>
      </div>
      <div
        class="schedule-preset {scheduleType === 'weekly' ? 'selected' : ''}"
        onclick={() => selectScheduleType('weekly')}
        onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && selectScheduleType('weekly')}
        role="button"
        tabindex="0"
      >
        <div style="margin-bottom: 4px;">📆</div>
        <div>Wöchentlich</div>
      </div>
      <div
        class="schedule-preset {scheduleType === 'monthly' ? 'selected' : ''}"
        onclick={() => selectScheduleType('monthly')}
        onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && selectScheduleType('monthly')}
        role="button"
        tabindex="0"
      >
        <div style="margin-bottom: 4px;">🗓️</div>
        <div>Monatlich</div>
      </div>
      <div
        class="schedule-preset {scheduleType === 'custom' ? 'selected' : ''}"
        onclick={() => selectScheduleType('custom')}
        onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && selectScheduleType('custom')}
        role="button"
        tabindex="0"
      >
        <div style="margin-bottom: 4px;">⚙️</div>
        <div>Benutzerdefiniert</div>
      </div>
      <div
        class="schedule-preset {scheduleType === 'manual' ? 'selected' : ''}"
        onclick={() => selectScheduleType('manual')}
        onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && selectScheduleType('manual')}
        role="button"
        tabindex="0"
      >
        <div style="margin-bottom: 4px;">🖐️</div>
        <div>Manuell</div>
      </div>
    </div>
  </div>
</div>

{#if scheduleType !== 'manual'}
  <div class="form-row">
    <div class="form-group">
      <label class="form-label" for="schedule-time">Zeit</label>
      <input id="schedule-time" class="form-input" type="time" bind:value={scheduleTime} />
      {#if errors.scheduleTime}<div class="error">{errors.scheduleTime}</div>{/if}
    </div>
    <div class="form-group">
      <label class="form-label" for="timezone">Zeitzone</label>
      <CustomSelect
        bind:value={scheduleTimezone}
        options={[
          { value: 'Local', label: 'Lokale Zeit' },
          { value: 'UTC', label: 'UTC' },
        ]}
      />
    </div>
  </div>
{/if}

<div class="info-box">⏰ Dieser Job wird {schedulePreview} ausgeführt</div>

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

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .schedule-builder {
    background: #1a1d2e;
    border: 1px solid #2d3348;
    border-radius: 8px;
    padding: 16px;
  }

  .schedule-quick {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 8px;
    margin-bottom: 16px;
  }

  .schedule-preset {
    padding: 10px;
    background: #22273a;
    border: 2px solid #2d3348;
    border-radius: 6px;
    cursor: pointer;
    text-align: center;
    font-size: 13px;
    transition: all 0.2s;
  }

  .schedule-preset:hover {
    border-color: #3b82f6;
  }

  .schedule-preset.selected {
    background: rgba(59, 130, 246, 0.15);
    border-color: #3b82f6;
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

    .schedule-quick {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>
