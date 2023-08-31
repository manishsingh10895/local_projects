<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Project from "./Project.svelte";
  import { getContext, onMount } from "svelte";
  import { get } from "svelte/store";
  let projects;

  onMount(() => {
    projects = getContext('projects');
  })
  async function refresh() {
    try {
      let projects = await invoke("get_projects");

      console.log(projects);
    } catch (err) {
      console.error(err);
    }
  }
</script>

<div class="view-container projects-container">
  <div class="projects">
    {#each projects as project}
      <div>
       <Project project={project}/> 
      </div>
    {/each}
  </div>

  <div class="documentation" />
</div>

<style>
  .projects {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
</style>