<script lang="ts">
  /**
   * Sidebar-Komponente mit Logo und Navigation.
   *
   * Zeigt das Rustic Logo und die Haupt-Navigation-Items an.
   * Unterstützt Active-State für die aktuelle Seite.
   *
   * @component
   *
   * @example
   * ```svelte
   * <Sidebar activePage="dashboard" />
   * ```
   */

  import { navigateTo } from '$lib/stores/router';

  // Props
  let {
    activePage = 'dashboard',
    mobileOpen = false,
    class: className = ''
  }: {
    activePage?: string;
    mobileOpen?: boolean;
    class?: string;
  } = $props();

  // Navigation-Items (entsprechend Mockup)
  const navItems = [
    { id: 'dashboard', label: 'Dashboard', icon: '📊' },
    { id: 'repositories', label: 'Repositories', icon: '📦' },
    { id: 'snapshots', label: 'Snapshots', icon: '⏱️' },
    { id: 'backup-jobs', label: 'Backup Jobs', icon: '🔄' },
    { id: 'settings', label: 'Settings', icon: '⚙️' }
  ];

  // Navigation Handler
  function handleNavigate(pageId: string) {
    navigateTo(pageId as any);
  }

  // Keyboard-Navigation für Accessibility
  function handleKeyDown(event: any, pageId: string) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handleNavigate(pageId);
    }
  }
</script>

<div class="sidebar {mobileOpen ? 'mobile-open' : ''} {className}" role="navigation" aria-label="Main navigation">
  <!-- Logo Section -->
  <div class="logo">
    <div class="logo-icon" aria-hidden="true">R</div>
    <div class="logo-text">
      <h1>Rustic</h1>
      <p>Backup Manager</p>
    </div>
  </div>

  <!-- Navigation Items -->
  <nav class="nav">
    {#each navItems as item (item.id)}
      <button
        class="nav-item"
        class:active={activePage === item.id}
        role="menuitem"
        tabindex="0"
        aria-label={item.label}
        aria-current={activePage === item.id ? 'page' : undefined}
        onclick={() => handleNavigate(item.id)}
        onkeydown={(e) => handleKeyDown(e, item.id)}
      >
        <span class="nav-icon" aria-hidden="true">{item.icon}</span>
        <span>{item.label}</span>
      </button>
    {/each}
  </nav>
</div>

<style>
  .sidebar {
    width: 280px;
    background: var(--bg-secondary);
    display: flex;
    flex-direction: column;
    border-right: 1px solid var(--border-color);
    flex-shrink: 0;
  }

  .logo {
    padding: 24px 20px;
    display: flex;
    align-items: center;
    gap: 12px;
    border-bottom: 1px solid var(--border-color);
  }

  .logo-icon {
    width: 40px;
    height: 40px;
    background: linear-gradient(135deg, var(--color-primary) 0%, var(--color-primary-dark) 100%);
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 20px;
    color: white;
  }

  .logo-text h1 {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .logo-text p {
    font-size: 12px;
    color: var(--text-secondary);
    margin: 0;
  }

  .nav {
    flex: 1;
    padding: 16px 12px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    border-radius: 8px;
    cursor: pointer;
    color: var(--text-secondary);
    font-size: 14px;
    margin-bottom: 4px;
    transition: all var(--transition-fast);
    border: none;
    background: none;
    width: 100%;
    text-align: left;
  }

  .nav-item:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .nav-item.active {
    background: var(--color-primary);
    color: white;
  }

  .nav-item:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }

  .nav-icon {
    font-size: 18px;
    width: 20px;
    flex-shrink: 0;
  }

  /* Mobile Styles */
  @media (max-width: 768px) {
    .sidebar {
      position: fixed;
      left: -280px;
      transition: left 0.3s;
      z-index: 100;
      height: 100vh;
    }

    .sidebar.mobile-open {
      left: 0;
    }
  }
</style>