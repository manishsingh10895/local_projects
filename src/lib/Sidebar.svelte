<script>
  import {Link, Router} from 'svelte-navigator';
    import LanguageLegend from './LanguageLegend.svelte';
</script>

<style lang="scss">
  
  .sidebar-container {
    height: 100vh;
    position: sticky;
    background: var(--tertiary-color);
    max-width: 300px;
    
  }

  .sidebar-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 10px;
    padding-top: 100px;
  }

  .spacer {
    flex-grow: 1;
  }

  :global(.sidebar-container a) {
    color: white;
  }


</style>

<nav class="sidebar-container"> 
  <Link to="/">Home</Link>
  <Link to="/add">Add Project Dir</Link>
  <div class="spacer"></div>

  <LanguageLegend/> 
</nav>
