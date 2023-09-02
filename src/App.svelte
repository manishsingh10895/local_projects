<script lang="ts">
  import { Route, Router } from "svelte-navigator";
  import Greet from "./lib/Greet.svelte";
  import { getContext, onMount, setContext } from "svelte";
  import Sidebar from "./lib/Sidebar.svelte";
  import ConfigView from "./lib/ConfigView.svelte";
  import Header from "./lib/Header.svelte";
  import ProjectsView from "./lib/ProjectsView.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { writable } from "svelte/store";
  import type { AppContext } from "./stores/AppContext";
  import type { Config } from "./types";

  const config = writable<Config>({
    project_dirs: [],
  });

  const projects = writable([]);

  setContext<AppContext>("appData", {
    config,
    projects,
  });

  const appData = getContext<AppContext>("appData");

  onMount(() => {
    invoke("get_projects")
      .then((projects) => {
        console.log(projects);
        appData.projects.set(projects as Array<any>);
      })
      .catch((err) => {
        console.error(err);
      });
  });
</script>

<main class="container">
  <Router>
    <div class="header-container">
      <Header />
    </div>
    <div class="sidebar-container">
      <Sidebar />
    </div>
    <div class="content">
      <div>
        <Route path="/">
          <ProjectsView />
        </Route>

        <Route path="/add">
          <ConfigView />
        </Route>
      </div>
    </div>
  </Router>
</main>

<style lang="scss">
  .container {
    display: grid;
    grid-template-columns: 150px 1fr;
    grid-template-rows: 50px 1fr;
    height: 100vh;
    grid-template-areas:
      "aside header"
      "aside content";
  }

  .sidebar-container {
    grid-area: aside;
  }
  .content {
    grid-area: content;
    padding-top: 10px;
    padding-left: 10px;
    padding-right: 10px;
    height: calc(100vh - 50px);
    overflow-x: hidden;
    overflow-y: auto;
  }
</style>
