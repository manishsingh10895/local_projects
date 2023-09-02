<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { appDir, homeDir } from "@tauri-apps/api/path";
  import { invoke } from "@tauri-apps/api/tauri";
  import { getContext, onMount } from "svelte";
  import type { AppContext } from "../stores/AppContext";
  import type { Config, IProject } from "../types";

  const appData = getContext<AppContext>("appData");

  let directories: Array<string> = [];

  onMount(() => {
    invoke<Config>("get_config")
      .then((config: Config) => {
        console.log(config);

        directories = config.project_dirs;
      })
      .catch((err) => {
        console.error(err);
      });
  });

  async function handleInput() {
    const selected = await open({
      directory: true,
      multiple: true,
      defaultPath: await homeDir(),
    });

    console.log(selected);

    try {
      if (!selected) {
        return;
      }

      for (let i = 0; i < selected.length; i++) {
        let path = selected[i];

        await invoke("config_add_dir", { path });
      }

      await invoke("reload_index");

      let projects = await invoke("get_projects");

      appData.projects.set(projects as Array<IProject>);
    } catch (error) {
      console.error(error);
    }
  }
</script>

<div class="config-container view-container">
  <div class="directories-container">
    <div class="sub-title">Directories</div>

    <div class="directories">
      {#each directories as dir}
        <div class="dir">
          {dir}
        </div>
      {/each}
    </div>
  </div>
  <div class="add-directory">
    <h3>Add a Directory</h3>

    <div class="input-container">
      <button on:click={handleInput}> Select Directory </button>
    </div>
  </div>
</div>

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
