<script lang="ts">
  import type { MouseEventHandler } from "svelte/elements";
  import { getIconForProject } from "../helpers/icon.helper";
  import type { IProject } from "../types";

  export let project: IProject;

  let projectIcon = getIconForProject(project.project_type.toString().toLowerCase());

  console.log(projectIcon);

  export let onClick: MouseEventHandler<HTMLDivElement>;
</script>

<div class="project" aria-roledescription="Click on Project" on:click={onClick}>
  <div class="logo-container">
    <img src={projectIcon} alt={project.project_type.toString()}/>
  </div>

  <div class="text-details">
    <div class="name">
      {project.name}
    </div>
    <div class="description">
      {project.description ? project.description : ''}
    </div>
  </div>
</div>

<style lang="scss">
  .project {
    background: rgb(171, 171, 171);
    background: linear-gradient(
      to right,
      rgb(222, 222, 222),
      rgb(214, 238, 255)
    );
    box-shadow: 0px -3px 26px 0px rgba(191, 191, 191, 1);
    border-radius: 16px;
    margin: 10px 0;
    padding: 15px 10px;
    display: flex;
    gap: 0.25rem;
    align-items: center;

    .logo-container {
        width: 50px;
        height: 100%;

        img {
            max-width: 100%;
        }
    }

    .name {
      font-size: 14px;
    }

    .description {
        font-size: 12px;
    }
  }
</style>
