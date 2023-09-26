<script lang="ts">
  import type { MouseEventHandler } from "svelte/elements";
  import { getIconForProject } from "../helpers/icon.helper";
  import type { IProject } from "../types";
  import Languages from "./Languages.svelte";
  import Icon from "svelte-icons-pack";
  import BsGithub from "svelte-icons-pack/bs/BsGithub";
  import { timeAgo } from "../helpers/time-ago.helper";
  import { invoke } from "@tauri-apps/api/tauri";

  export let project: IProject;

  let projectIcon = getIconForProject(
    project.project_type.toString().toLowerCase()
  );

  console.log(projectIcon);

  export let onViewDoc: Function;

  function onKeyDown(e) {
    if (e.keyCode === 13) {
      onViewDoc(project);
    }
  }

  function onOpen() {
    invoke("open_project", { path: project.path });
  }

  let time = new Date(project.last_modified.secs_since_epoch * 1000);
</script>

<div
  class="project"
  tabindex="0"
  role="button"
  aria-roledescription="Click on Project"
>
  {#if project.git && project.git.length}
    <span
      aria-roledescription="Github Repo"
      role="link"
      class="git"
    >
      <a target="_blank" href={project.git[0]}>
        <Icon src={BsGithub} />
      </a>
    </span>
  {/if}

  <div class="details-container">
    <div class="logo-container">
      <img src={projectIcon} alt={project.project_type.toString()} />
    </div>

    <div class="text-details">
      <div class="name">
        {project.name}
      </div>
      <div class="description">
        {project.description ? project.description : ""}
      </div>
    </div>
  </div>

  <div class="languages-container">
    <Languages languages={project.language_map} />
  </div>

  <div class="actions">
    {#if project.git.length}
      <button
        class="lp_button sm"
        on:keydown={onKeyDown}
        on:click={(_) => onViewDoc(project)}
      >
        View Doc
      </button>
    {/if}
    <button on:click={onOpen} class="lp_button secondary sm"> Open </button>
  </div>

  <div class="time">
    {timeAgo(time)}
  </div>
</div>

<style lang="scss">
  .project {
    position: relative;
    cursor: pointer;
    background: rgb(171, 171, 171);
    background: linear-gradient(
      to right,
      rgb(222, 222, 222),
      rgb(214, 238, 255)
    );
    box-shadow: 0 -1px 7px -1px rgb(191, 191, 191);
    border-radius: 16px;
    margin: 10px 0;
    padding: 15px 10px;
    display: flex;
    gap: 0.25rem;
    min-width: 250px;
    max-width: 250px;

    flex-direction: column;

    .git {
      position: absolute;
      top: 10px;
      right: 10px;
    }

    .actions {
      display: flex;
      gap: 10px;
      justify-content: center;
      align-items: center;
      margin: 10px 0 ;
    }

    .time {
      position: absolute;
      bottom: 5px;
      left: 10px;
      font-size: 10px;
      color: gray;
    }

    .details-container {
      display: flex;
      align-items: center;
      column-gap: 0.25rem;

      .logo-container {
        width: 50px;
        height: 100%;

        img {
          max-width: 100%;
        }
      }

      .name {
        font-size: 16px;
        margin-bottom: 10px;
      }

      .description {
        font-size: 12px;
        color: var(--text-color);
        word-break: break-word;
      }
    }
  }

  .languages-container {
    height: 10px;
  }
</style>
