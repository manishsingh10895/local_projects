<script lang="ts">
  import { Route, Router } from "svelte-navigator";
  import Greet from "./lib/Greet.svelte";
  import { getContext, onMount, setContext } from "svelte";
  import Sidebar from "./lib/Sidebar.svelte";
  import ConfigView from "./lib/ConfigView.svelte";
  import Header from "./lib/Header.svelte";
  import ProjectsView from "./lib/ProjectsView.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let projects;

  onMount(() => {
    invoke('get_projects')
      .then((projects) => {
        console.log(projects);
        projects = projects;
        setContext('projects', projects);
      })
      .catch(err => {
        console.error(err);
      })
  });
</script>

<main class="container">
  <h4>Container</h4>
  <!-- {projects.len} -->
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
    grid-template-columns: 300px 1fr;
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
  }
</style>
