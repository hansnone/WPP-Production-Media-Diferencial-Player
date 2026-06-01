<script lang="ts">
  import { crearComandos, filtrarComandos, type ComandoPaleta } from "../../command-palette/commands";

  interface Props {
    abierta: boolean;
    oncerrar: () => void;
    fabrica: Parameters<typeof crearComandos>[0];
    placeholder?: string;
    tituloDialogo?: string;
  }

  let {
    abierta,
    oncerrar,
    fabrica,
    placeholder = "Buscar comando…",
    tituloDialogo = "Paleta de comandos",
  }: Props = $props();

  let consulta = $state("");
  let indice = $state(0);

  const todos = $derived(crearComandos(fabrica));
  const filtrados = $derived(filtrarComandos(todos, consulta));

  $effect(() => {
    if (abierta) {
      consulta = "";
      indice = 0;
    }
  });

  $effect(() => {
    if (indice >= filtrados.length) indice = Math.max(0, filtrados.length - 1);
  });

  function ejecutar(cmd: ComandoPaleta) {
    oncerrar();
    void cmd.ejecutar();
  }

  function onKeydown(e: KeyboardEvent) {
    if (!abierta) return;
    if (e.key === "Escape") {
      e.preventDefault();
      oncerrar();
    }
    if (e.key === "ArrowDown") {
      e.preventDefault();
      indice = Math.min(indice + 1, filtrados.length - 1);
    }
    if (e.key === "ArrowUp") {
      e.preventDefault();
      indice = Math.max(indice - 1, 0);
    }
    if (e.key === "Enter" && filtrados[indice]) {
      e.preventDefault();
      ejecutar(filtrados[indice]);
    }
  }
</script>

<svelte:window onkeydown={onKeydown} />

{#if abierta}
  <div class="backdrop" role="presentation" onclick={oncerrar} data-testid="command-palette">
    <div
      class="dialogo"
      role="dialog"
      aria-modal="true"
      aria-label={tituloDialogo}
      tabindex="-1"
      onkeydown={(e) => e.stopPropagation()}
      onclick={(e) => e.stopPropagation()}
    >
      <input
        class="entrada"
        type="search"
        {placeholder}
        bind:value={consulta}
        data-testid="command-palette-input"
        autofocus
      />
      <ul class="lista" role="listbox">
        {#each filtrados as cmd, i (cmd.id)}
          <li>
            <button
              type="button"
              class="item"
              class:item--activo={i === indice}
              role="option"
              aria-selected={i === indice}
              onclick={() => ejecutar(cmd)}
            >
              <span class="item__grupo">{cmd.grupo}</span>
              <span class="item__etiqueta">{cmd.etiqueta}</span>
              {#if cmd.atajo}
                <kbd class="item__atajo">{cmd.atajo}</kbd>
              {/if}
            </button>
          </li>
        {:else}
          <li class="vacio">Sin resultados</li>
        {/each}
      </ul>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.55);
    z-index: 1000;
    display: flex;
    justify-content: center;
    padding-top: 12vh;
  }
  .dialogo {
    width: min(520px, 92vw);
    background: var(--bg-elev);
    border: 1px solid var(--border);
    border-radius: var(--radius-max);
    box-shadow: 0 0 0 1px var(--border);
    overflow: hidden;
  }
  .entrada {
    width: 100%;
    box-sizing: border-box;
    padding: 12px 14px;
    background: var(--bg-app);
    border: none;
    border-bottom: 1px solid var(--border);
    color: var(--text-primary);
    font-size: 14px;
    outline: none;
  }
  .entrada:focus {
    box-shadow: inset 0 0 0 1px var(--accent-primary);
  }
  .lista {
    list-style: none;
    margin: 0;
    padding: 4px;
    max-height: 320px;
    overflow-y: auto;
  }
  .item {
    width: 100%;
    display: grid;
    grid-template-columns: 80px 1fr auto;
    gap: 8px;
    align-items: center;
    padding: 8px 10px;
    background: transparent;
    border: none;
    color: var(--text-primary);
    text-align: left;
    border-radius: var(--radius);
    cursor: pointer;
    font-size: 13px;
  }
  .item:hover,
  .item--activo {
    background: var(--bg-hover);
  }
  .item__grupo {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    color: var(--text-muted);
  }
  .item__atajo {
    font-family: var(--font-mono);
    font-size: 10px;
    padding: 2px 6px;
    background: var(--bg-darkest);
    border: 1px solid var(--border);
    border-radius: var(--radius);
  }
  .vacio {
    padding: 12px;
    color: var(--text-muted);
    font-size: 12px;
  }
</style>
