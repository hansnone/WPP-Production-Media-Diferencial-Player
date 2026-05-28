<script lang="ts">
  import {
    FolderOpen,
    Pause,
    Play,
    SkipBack,
    SkipForward,
    Search,
  } from "@lucide/svelte";
  import IconBtn from "../controls/IconBtn.svelte";
  import { playerStore } from "../../stores/player.svelte";
  import { formatearPts } from "../../player";

  interface Props {
    onPaleta: () => void;
    seekInput: number;
    onSeekInput: (v: number) => void;
    onSeek: () => void;
  }

  let { onPaleta, seekInput = $bindable(0), onSeek, onSeekInput }: Props = $props();

  const duracionMax = $derived(
    Math.max(playerStore.snap?.duracion_a ?? 0, playerStore.snap?.duracion_b ?? 0),
  );
</script>

<div class="toolbar" data-testid="toolbar">
  <div class="toolbar__grupo">
    <IconBtn titulo="Abrir A" onclick={() => playerStore.abrir("a")}>
      <FolderOpen size={16} strokeWidth={1.5} />
    </IconBtn>
    <IconBtn titulo="Abrir B" onclick={() => playerStore.abrir("b")}>
      <FolderOpen size={16} strokeWidth={1.5} color="var(--chan-b)" />
    </IconBtn>
    <span class="toolbar__sep"></span>
    <IconBtn
      titulo="Play / Pausa"
      onclick={() => playerStore.playPausa()}
      activo={playerStore.snap?.reproduciendo}
    >
      {#if playerStore.snap?.reproduciendo}
        <Pause size={16} strokeWidth={1.5} />
      {:else}
        <Play size={16} strokeWidth={1.5} />
      {/if}
    </IconBtn>
    <IconBtn titulo="Frame atrás" onclick={() => playerStore.stepBck()}>
      <SkipBack size={16} strokeWidth={1.5} />
    </IconBtn>
    <IconBtn titulo="Frame adelante" onclick={() => playerStore.stepFwd()}>
      <SkipForward size={16} strokeWidth={1.5} />
    </IconBtn>
  </div>

  <div class="toolbar__transporte">
    <span class="mono tiempo">
      {formatearPts(playerStore.snap?.pts_actual ?? 0)} / {formatearPts(duracionMax)}
    </span>
    <input
      type="range"
      class="slider"
      min="0"
      max={duracionMax || 1}
      step="0.001"
      value={seekInput}
      oninput={(e) => onSeekInput(Number((e.target as HTMLInputElement).value))}
    />
    <button type="button" class="btn-seek" onclick={onSeek}>Seek</button>
  </div>

  <IconBtn titulo="Paleta (Ctrl+K)" onclick={onPaleta}>
    <Search size={16} strokeWidth={1.5} />
  </IconBtn>
</div>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 4px 8px;
    background: var(--bg-app);
    border-bottom: 1px solid var(--border);
    min-height: 36px;
  }
  .toolbar__grupo {
    display: flex;
    align-items: center;
    gap: 2px;
  }
  .toolbar__sep {
    width: 1px;
    height: 20px;
    background: var(--border);
    margin: 0 4px;
  }
  .toolbar__transporte {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }
  .tiempo {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
  }
  .slider {
    flex: 1;
    height: 2px;
    accent-color: var(--accent-primary);
  }
  .btn-seek {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    padding: 4px 8px;
    background: var(--bg-panel);
    border: 1px solid var(--border);
    color: var(--text-primary);
    border-radius: var(--radius);
    cursor: pointer;
  }
  .btn-seek:hover {
    background: var(--bg-hover);
  }
</style>
