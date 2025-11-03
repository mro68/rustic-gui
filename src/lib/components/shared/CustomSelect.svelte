<!-- CustomSelect.svelte: Custom Select mit vollständiger Dark-Theme-Unterstützung -->
<script lang="ts">
  /**
   * Custom Select Dropdown mit vollständiger Styling-Kontrolle.
   * Ersetzt natives <select>, da Browser <option> background/color ignorieren.
   *
   * @component
   *
   * @example
   * ```svelte
   * <CustomSelect
   *   bind:value={selectedValue}
   *   options={[
   *     { value: 'dark', label: 'Dark' },
   *     { value: 'light', label: 'Light' }
   *   ]}
   *   placeholder="Wähle..."
   * />
   * ```
   */

  export interface SelectOption {
    value: string;
    label: string;
    disabled?: boolean;
  }

  let {
    value = $bindable(''),
    options = [],
    placeholder = 'Auswählen...',
    disabled = false,
    class: className = '',
  }: {
    value?: string;
    options: SelectOption[];
    placeholder?: string;
    disabled?: boolean;
    class?: string;
  } = $props();

  let isOpen = $state(false);
  let selectRef: HTMLDivElement;

  // Aktuell ausgewählte Option
  const selectedOption = $derived(options.find((opt) => opt.value === value) || null);

  const displayText = $derived(selectedOption ? selectedOption.label : placeholder);

  function toggleDropdown() {
    if (!disabled) {
      isOpen = !isOpen;
    }
  }

  function selectOption(option: SelectOption) {
    if (!option.disabled) {
      value = option.value;
      isOpen = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (disabled) return;

    switch (event.key) {
      case 'Enter':
      case ' ':
        event.preventDefault();
        toggleDropdown();
        break;
      case 'Escape':
        isOpen = false;
        break;
      case 'ArrowDown':
        event.preventDefault();
        if (!isOpen) {
          isOpen = true;
        } else {
          // Navigate to next option
          const currentIndex = options.findIndex((opt) => opt.value === value);
          const nextIndex = currentIndex + 1;
          if (nextIndex < options.length && !options[nextIndex].disabled) {
            value = options[nextIndex].value;
          }
        }
        break;
      case 'ArrowUp':
        event.preventDefault();
        if (isOpen) {
          const currentIndex = options.findIndex((opt) => opt.value === value);
          const prevIndex = currentIndex - 1;
          if (prevIndex >= 0 && !options[prevIndex].disabled) {
            value = options[prevIndex].value;
          }
        }
        break;
    }
  }

  function handleClickOutside(event: MouseEvent) {
    if (selectRef && !selectRef.contains(event.target as Node)) {
      isOpen = false;
    }
  }

  $effect(() => {
    if (isOpen) {
      document.addEventListener('click', handleClickOutside);
      return () => document.removeEventListener('click', handleClickOutside);
    }
  });
</script>

<div
  bind:this={selectRef}
  class="custom-select {className}"
  class:disabled
  class:open={isOpen}
  role="button"
  tabindex={disabled ? -1 : 0}
  aria-haspopup="listbox"
  aria-expanded={isOpen}
  onkeydown={handleKeydown}
  onclick={toggleDropdown}
>
  <div class="select-trigger">
    <span class="select-value" class:placeholder={!selectedOption}>
      {displayText}
    </span>
    <span class="select-arrow" class:open={isOpen}>▼</span>
  </div>

  {#if isOpen}
    <div class="select-dropdown" role="listbox">
      {#each options as option (option.value)}
        <div
          class="select-option"
          class:selected={option.value === value}
          class:disabled={option.disabled}
          role="option"
          aria-selected={option.value === value}
          tabindex={option.disabled ? -1 : 0}
          onclick={() => selectOption(option)}
          onkeydown={(e) => e.key === 'Enter' && selectOption(option)}
        >
          {option.label}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .custom-select {
    position: relative;
    width: 100%;
    user-select: none;
  }

  .select-trigger {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 0.375rem;
    color: var(--text-primary);
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s;
    min-height: 2.75rem;
  }

  .custom-select:focus .select-trigger,
  .select-trigger:hover {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
  }

  .custom-select.disabled .select-trigger {
    opacity: 0.5;
    cursor: not-allowed;
    background: var(--bg-secondary);
  }

  .select-value {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .select-value.placeholder {
    color: var(--text-secondary);
  }

  .select-arrow {
    margin-left: 0.5rem;
    font-size: 0.75rem;
    color: var(--text-secondary);
    transition: transform 0.2s;
  }

  .select-arrow.open {
    transform: rotate(180deg);
  }

  .select-dropdown {
    position: absolute;
    top: calc(100% + 0.25rem);
    left: 0;
    right: 0;
    max-height: 300px;
    overflow-y: auto;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 0.375rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    z-index: 1000;
    animation: fadeIn 0.15s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .select-option {
    padding: 0.75rem;
    color: var(--text-primary);
    background: var(--bg-secondary);
    cursor: pointer;
    transition: all 0.15s;
  }

  .select-option:hover {
    background: var(--bg-tertiary);
  }

  .select-option.selected {
    background: var(--color-primary);
    color: white;
    font-weight: 500;
  }

  .select-option.selected:hover {
    background: var(--color-primary-dark);
  }

  .select-option.disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background: var(--bg-secondary);
  }

  .select-option.disabled:hover {
    background: var(--bg-secondary);
  }

  /* Scrollbar Styling */
  .select-dropdown::-webkit-scrollbar {
    width: 8px;
  }

  .select-dropdown::-webkit-scrollbar-track {
    background: var(--bg-primary);
    border-radius: 0.375rem;
  }

  .select-dropdown::-webkit-scrollbar-thumb {
    background: var(--bg-tertiary);
    border-radius: 0.375rem;
  }

  .select-dropdown::-webkit-scrollbar-thumb:hover {
    background: var(--border-color);
  }
</style>
