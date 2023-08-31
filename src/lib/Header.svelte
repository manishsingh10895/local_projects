<style lang="scss">
  .header {
    background: var(--primary-color);
    display:flex;
    justify-content: space-between;
    padding: 10px 15px;
    h3 {
      font-size: 2rem;
      font-weight: bold;
      color: var(--text-color-alt);
      margin: 0;
    }
  }
</style>

<div class="header">
  <h3>
     Local Projects 
  </h3> 
    <div class="actions">
    <button class="lp_button secondary" on:click={reloadIndex}>
      Reload Index
    </button>
  </div>
</div>

  <script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

   async function reloadIndex() {
      console.log('Reloading Index');
    await invoke('reload_index')
   } 
</script>
