<script lang="ts">
  import { idiomaStore } from "../../i18n/idioma.svelte";
  import { layoutStore } from "../../stores/layout.svelte";
  import { playerStore } from "../../stores/player.svelte";
  import { recientesStore } from "../../stores/recientes.svelte";

  interface Props {
    onPaleta: () => void;
  }

  let { onPaleta }: Props = $props();

  const t = (clave: Parameters<typeof idiomaStore.t>[0]) => idiomaStore.t(clave);
</script>

<menubar class="menubar" data-testid="menubar">
  <div class="menubar__grupo">
    <span class="menubar__marca">DiffPlayerQC</span>
  </div>
  <div class="menubar__menus">
    <details class="menu">
      <summary>{t("menu.archivo")}</summary>
      <div class="menu__dropdown" data-testid="menu-archivo">
        <button type="button" onclick={() => playerStore.abrir("a")}>{t("menu.abrirA")}</button>
        <button type="button" onclick={() => playerStore.abrir("b")}>{t("menu.abrirB")}</button>
        <span class="menu__sep"></span>
        <span class="menu__subtitulo">{t("menu.recientes")}</span>
        {#if recientesStore.entradas.length === 0}
          <span class="menu__hint" data-testid="menu-sin-recientes">{t("menu.sinRecientes")}</span>
        {:else}
          <div class="menu__recientes" data-testid="menu-recientes">
            {#each recientesStore.entradas as ent (ent.ruta)}
              <button
                type="button"
                class="menu__reciente"
                title={ent.ruta}
                onclick={() => playerStore.abrirReciente(ent.ruta, ent.canal)}
              >
                <span class="menu__reciente-canal">{ent.canal.toUpperCase()}</span>
                <span class="menu__reciente-nombre">{ent.nombre}</span>
              </button>
            {/each}
          </div>
          <button type="button" class="menu__secundario" onclick={() => recientesStore.vaciar()}>
            {t("menu.limpiarRecientes")}
          </button>
        {/if}
      </div>
    </details>
    <details class="menu" data-testid="menu-ver">
      <summary>{t("menu.ver")}</summary>
      <div class="menu__dropdown">
        <button type="button" onclick={onPaleta}>{t("menu.paleta")}</button>
        <button type="button" onclick={() => layoutStore.resetearWorkspaceActual()}>
          {t("menu.resetLayout")}
        </button>
        <span class="menu__sep"></span>
        <span class="menu__subtitulo">{t("menu.idioma")}</span>
        <button
          type="button"
          class:menu__activo={idiomaStore.idioma === "es"}
          data-testid="menu-idioma-es"
          onclick={() => idiomaStore.establecer("es")}
        >
          {t("menu.idiomaEs")}
        </button>
        <button
          type="button"
          class:menu__activo={idiomaStore.idioma === "en"}
          data-testid="menu-idioma-en"
          onclick={() => idiomaStore.establecer("en")}
        >
          {t("menu.idiomaEn")}
        </button>
      </div>
    </details>
    <details class="menu">
      <summary>{t("menu.ayuda")}</summary>
      <div class="menu__dropdown">
        <span class="menu__hint">{t("menu.ayudaVersion")}</span>
      </div>
    </details>
  </div>
</menubar>

<style>
  .menubar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 28px;
    padding: 0 8px;
    background: var(--bg-darkest);
    border-bottom: 1px solid var(--border);
    font-size: 12px;
  }
  .menubar__marca {
    font-weight: 600;
    letter-spacing: var(--letter-label);
    text-transform: uppercase;
    font-size: 10px;
    color: var(--text-muted);
  }
  .menubar__menus {
    display: flex;
    gap: 4px;
  }
  .menu {
    position: relative;
  }
  .menu summary {
    list-style: none;
    cursor: pointer;
    padding: 4px 8px;
    color: var(--text-primary);
  }
  .menu summary::-webkit-details-marker {
    display: none;
  }
  .menu__dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    min-width: 220px;
    max-width: 360px;
    background: var(--bg-elev);
    border: 1px solid var(--border);
    border-radius: var(--radius-max);
    z-index: 50;
    display: flex;
    flex-direction: column;
    padding: 4px;
  }
  .menu__dropdown button {
    text-align: left;
    padding: 6px 10px;
    background: transparent;
    border: none;
    color: var(--text-primary);
    cursor: pointer;
    font-size: 12px;
    border-radius: var(--radius);
  }
  .menu__dropdown button:hover {
    background: var(--bg-hover);
  }
  .menu__activo {
    color: var(--accent-primary);
  }
  .menu__sep {
    height: 1px;
    background: var(--border);
    margin: 4px 0;
  }
  .menu__subtitulo {
    padding: 4px 10px 2px;
    font-size: 9px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    color: var(--text-muted);
  }
  .menu__hint {
    padding: 6px 10px;
    color: var(--text-muted);
    font-size: 11px;
  }
  .menu__secundario {
    font-size: 11px !important;
    color: var(--text-muted) !important;
  }
  .menu__recientes {
    max-height: 200px;
    overflow-y: auto;
  }
  .menu__reciente {
    display: flex !important;
    align-items: center;
    gap: 8px;
  }
  .menu__reciente-canal {
    font-size: 9px;
    font-weight: 700;
    color: var(--accent-primary);
    flex-shrink: 0;
  }
  .menu__reciente-nombre {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
