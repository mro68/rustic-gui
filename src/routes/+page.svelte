<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let name = $state('');
  let greetMsg = $state('');

  // Repository-Test States
  let repoPath = $state('/tmp/test-repo');
  let repoPassword = $state('test-password');
  let repoResult = $state('');
  let repoError = $state('');

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke('greet', { name });
  }

  async function testInitRepository() {
    try {
      repoError = '';
      const result = await invoke('init_repository', {
        path: repoPath,
        password: repoPassword,
        backendType: 'local',
        backendOptions: null
      });
      repoResult = `Repository initialisiert: ${JSON.stringify(result, null, 2)}`;
    } catch (error) {
      repoError = `Fehler: ${error}`;
    }
  }

  async function testOpenRepository() {
    try {
      repoError = '';
      const result = await invoke('open_repository', {
        path: repoPath,
        password: repoPassword
      });
      repoResult = `Repository geöffnet: ${JSON.stringify(result, null, 2)}`;
    } catch (error) {
      repoError = `Fehler: ${error}`;
    }
  }

  async function testCheckRepository() {
    try {
      repoError = '';
      const result = await invoke('check_repository', {
        path: repoPath,
        password: repoPassword
      });
      repoResult = `Repository-Check: ${JSON.stringify(result, null, 2)}`;
    } catch (error) {
      repoError = `Fehler: ${error}`;
    }
  }
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>

  <div class="row">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>
  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>

  <!-- Repository Test Section -->
  <div class="repository-test">
    <h2>Repository-Management Test</h2>
    <p>Teste die grundlegenden Repository-Funktionen:</p>

    <div class="row">
      <input
        id="repo-path"
        placeholder="Repository-Pfad (z.B. /tmp/test-repo)"
        bind:value={repoPath}
      />
      <input
        id="repo-password"
        type="password"
        placeholder="Passwort"
        bind:value={repoPassword}
      />
    </div>

    <div class="row">
      <button onclick={testInitRepository}>Repository initialisieren</button>
      <button onclick={testOpenRepository}>Repository öffnen</button>
      <button onclick={testCheckRepository}>Repository prüfen</button>
    </div>

    {#if repoResult}
      <div class="result success">
        <pre>{repoResult}</pre>
      </div>
    {/if}

    {#if repoError}
      <div class="result error">
        <pre>{repoError}</pre>
      </div>
    {/if}
  </div>
</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte-kit:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }

  .repository-test {
    margin-top: 2rem;
    padding: 1.5rem;
    border: 1px solid #ccc;
    border-radius: 8px;
    background-color: #f9f9f9;
  }

  .repository-test h2 {
    margin-top: 0;
    color: #333;
  }

  .repository-test .row {
    margin-bottom: 1rem;
  }

  .repository-test input {
    flex: 1;
    margin-right: 0.5rem;
  }

  .repository-test button {
    margin-right: 0.5rem;
  }

  .result {
    margin-top: 1rem;
    padding: 1rem;
    border-radius: 4px;
    font-family: monospace;
    white-space: pre-wrap;
  }

  .result.success {
    background-color: #d4edda;
    border: 1px solid #c3e6cb;
    color: #155724;
  }

  .result.error {
    background-color: #f8d7da;
    border: 1px solid #f5c6cb;
    color: #721c24;
  }

  .result pre {
    margin: 0;
    font-size: 0.9rem;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
