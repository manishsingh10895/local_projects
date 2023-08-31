<script>
  import {open} from '@tauri-apps/api/dialog';
  import { appDir, homeDir} from '@tauri-apps/api/path';

  async function handleInput() {
    const selected = await open({
      directory: true,
      multiple: true,
      defaultPath: await homeDir()
    }) ;

    console.log(selected);
  }
</script>

<style lang="scss">
  .config-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    .config {

    }
  }

  .input-container {
    border-radius: 16px;
    background: var(--secondary-color);
    padding: 15px;
    color: white;
  }
</style>

<div class="config-container view-container">
 <div class="config">
    <h3>
      Add a Directory
    </h3>

    <div class="input-container">
      <button on:click={handleInput}>
        Select Directory
      </button>
    </div>
  </div> 
</div>
