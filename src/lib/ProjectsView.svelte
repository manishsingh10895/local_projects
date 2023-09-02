<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { getContext, onMount } from "svelte";
  import type { AppContext } from "../stores/AppContext";
  import type { IProject } from "../types";
  import Project from "./Project.svelte";
  import DocView from "./DocView.svelte";

  const appContext = getContext<AppContext>("appData");
  let projects: Array<IProject> = [];

  appContext.projects.subscribe((ps) => {
    projects = ps;
  });

  let activeProject: IProject;

  let isDocViewOpen: boolean = false;

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

  $: {
    console.log("[isDocViewOpen] -> ", isDocViewOpen);
    if (isDocViewOpen) {
    } else {
    }
  }
</script>

<div class="view-container projects-container">
  <div class="projects">
    {#each projects as project}
        <Project
          {project}
          handleClick={() => {
            activeProject = project;
            isDocViewOpen = true;
          }}
        />
    {/each}
  </div>

  <div
    class="doc-bar"
    style:transform={`translateX(${isDocViewOpen ? "0" : "110%"})`}
  >
    <div on:keydown={(e) => {}} class="doc-bar__inner">
      <span
        style:cursor="pointer"
        role="button"
        on:click={() => {
          isDocViewOpen = false;
        }}
        on:keydown={(e) => {
          if (e.keyCode == 13) {
            isDocViewOpen = false;
          }
        }}
        class="close"
      >
        X
      </span>
      {#if activeProject}
        <DocView file={activeProject.documentation_file} />
      {/if}
    </div>
  </div>
</div>

<style lang="scss">
  .projects {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    column-gap: 1rem;
  }

  .projects-container {
    display: flex;
    justify-content: stretch;
    position: relative;
    gap: 0.5rem;
    .projects {
    }

    .doc-bar {
      min-width: 300px;
      max-width: 35vw;
      padding: 10px 15px;
      height: 100%;
      flex-grow: 1;
      transition: 0.3s all ease-in;
      position: absolute;
      right: 0;
      top: 0;
      background: #e0f2f1;
      transform: translateX(110%);
      z-index: 10;

      .close {
        border-radius: 25px;
        padding: 5px;
        color: white;
        background-color: var(--secondary-color);
      }
    }
  }
</style>
