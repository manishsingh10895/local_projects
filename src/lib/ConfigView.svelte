<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { appDir, homeDir } from "@tauri-apps/api/path";
  import { invoke } from "@tauri-apps/api/tauri";
  import { getContext, onMount } from "svelte";
  import type { AppContext } from "../stores/AppContext";
  import type { Config, IProject } from "../types";

  const appData = getContext<AppContext>("appData");

  let directories: Array<string> = [];

  let isIndexing = false;

  async function checkIfIndexing() {
    try {
      let indexing: boolean = await invoke("is_indexing");
      console.log(indexing);

      isIndexing = indexing;

      return indexing;
    } catch (err) {
      console.error(err);
    }
  }

  function startIndexingTimer() {
    let interval = setInterval(async () => {
      let indexing = await checkIfIndexing();

      if (!indexing) {
        clearInterval(interval);
      }
    }, 500);
  }

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
      await invoke("re_index");

      startIndexingTimer();

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
          <span>{dir}</span>
          <span class="close">x</span>
        </div>
      {/each}
    </div>
  </div>
  {#if isIndexing}
    <div class="loader">Indexing ....</div>
  {/if}
  <div class="add-directory">
    <h3>Add a Directory</h3>

    <div class="input-container">
      <button class="lp_button" on:click={handleInput}>
        Select Directory
      </button>
    </div>
  </div>
</div>

<style lang="scss">
  .config-container {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;
    .config {
    }
  }

  .input-container {
    border-radius: 16px;
    padding: 5px;
    text-align: center;
  }

  .loader {
    height: 150px;
    width: 100%;
    display: grid;
    place-items: center;

    font-size: 22px;
  }

  .directories {
    .dir {
      font-size: 1.2rem;
      padding: 8px 4px;
    }

    .close {
      color: red;
      cursor: pointer;
      font-size: 1.6rem;
      margin-left: 10px;
    }
  }

  .sub-title {
    border-bottom: 1px solid var(--primary-color);
  }
</style>
