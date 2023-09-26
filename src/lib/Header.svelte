<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { getContext } from "svelte";
  import type { AppContext } from "../stores/AppContext";

  const appContext = getContext<AppContext>("appData");

  async function refresh() {
    try {
      let projects = await invoke("get_projects");
      console.log(projects);
      appContext.projects.set(projects as Array<any>);
    } catch (err) {
      console.error(err);
    }
  }

  async function reloadIndex() {
    console.log("Reloading Index");
    await invoke("reload_index");

    refresh();
  }
</script>

<div class="header">
  <h3>Local Projects</h3>
  <div class="actions">
    <button class="lp_button secondary" on:click={reloadIndex}>
      Reload Index
    </button>
  </div>
</div>

<style lang="scss">
  .header {
    background: var(--primary-color);
    display: flex;
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
