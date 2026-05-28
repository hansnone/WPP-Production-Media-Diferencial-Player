<script lang="ts">
  import { etiquetaWorkspace } from "./lib/workspaces";

  let workspace = $state("compare");
</script>

<main class="shell">
  <header>
    <h1>DiffPlayerQC v2</h1>
    <p class="sub">M0 — Bootstrap (Tauri + Svelte 5)</p>
  </header>
  <section>
    <p>Workspace activo: <strong>{etiquetaWorkspace(workspace)}</strong></p>
    <p class="hint">La reproducción y el canvas llegan en M1–M3.</p>
  </section>
</main>

<style>
  .shell {
    font-family: system-ui, sans-serif;
    background: #0f1115;
    color: #e8eaed;
    min-height: 100vh;
    padding: 2rem;
  }
  h1 {
    font-size: 1.5rem;
    margin: 0;
  }
  .sub {
    color: #8b929a;
    margin-top: 0.25rem;
  }
  .hint {
    color: #6b7280;
    font-size: 0.9rem;
  }
</style>
