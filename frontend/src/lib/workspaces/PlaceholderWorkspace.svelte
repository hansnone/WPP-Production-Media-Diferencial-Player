<script lang="ts">
  interface Props {
    id: string;
    titulo: string;
    mensaje: string;
  }
  let { id, titulo, mensaje }: Props = $props();
</script>

<div class="placeholder" data-testid="workspace-{id}">
  <h2>{titulo}</h2>
  <p>{mensaje}</p>
</div>

<style>
  .placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-muted);
    gap: 8px;
  }
  h2 {
    font-size: 14px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    margin: 0;
    color: var(--text-primary);
  }
  p {
    font-size: 12px;
    margin: 0;
  }
</style>
