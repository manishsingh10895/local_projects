<script lang="ts">
    import { getLanguageColor } from "../helpers/lang-color.helper";

  export let languages: Map<String, number>;
</script>

<div class="languages-line-container">
  <div class="languages">
    {#each Object.keys(languages) as lang}
      <div style={`width: ${languages[lang]}%; background-color: ${getLanguageColor(lang.toLowerCase())};`} class="lang" />
    {/each}
  </div>
</div>

<style>
  .lang {
    height: 4px;
  }

  .languages-line-container {

  }

  .languages {
    display: flex;
    flex-wrap: nowrap;
  }
</style>

