<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { getContext, onMount } from "svelte";
  import type { AppContext } from "../stores/AppContext";
  import type { IProject } from "../types";
  import Project from "./Project.svelte";

  const appContext = getContext<AppContext>("appData");
  let projects: Array<IProject> = [];
  appContext.projects.subscribe((ps) => {
    projects = ps;
  });

  onMount(() => {});
  async function refresh() {
    try {
      let projects = await invoke("get_projects");
      console.log(projects);
      appContext.projects.set(projects as Array<any>);
    } catch (err) {
      console.error(err);
    }
  }
</script>

<div class="view-container projects-container">
  <div class="projects">
    {#each projects as project}
      <div>
        <Project {project} />
      </div>
    {/each}
  </div>

  <div class="documentation" >

  </div>
</div>

<style lang="scss">
  .projects {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    max-width: 250px;
  }

  .projects-container {
    display: flex;
    justify-content: stretch;

    .projects {
      flex-basis: 50%;
    }

    .documentation {
      flex-grow: 1;
    }
  }
</style>
