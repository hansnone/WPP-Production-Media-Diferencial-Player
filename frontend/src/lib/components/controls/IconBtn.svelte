<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    titulo: string;
    activo?: boolean;
    onclick?: () => void;
    children: Snippet;
  }

  let { titulo, activo = false, onclick, children }: Props = $props();
</script>

<button
  type="button"
  class="icon-btn"
  class:activo
  title={titulo}
  aria-label={titulo}
  {onclick}
>
  {@render children()}
</button>

<style>
  .icon-btn {
    width: 28px;
    height: 28px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: var(--text-primary);
    border-radius: var(--radius);
    cursor: pointer;
    padding: 0;
  }
  .icon-btn:hover {
    background: var(--bg-hover);
  }
  .icon-btn.activo {
    background: var(--accent-primary);
    color: #fff;
  }
</style>
