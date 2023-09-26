<script lang="ts">
  import { LANGUAGE_COLOR_MAP } from "../helpers/lang-color.helper";
</script>

<div class="legends">
  {#each Object.entries(LANGUAGE_COLOR_MAP) as [lang, color]}
    <div class="lang">
      <div>{lang}</div>
      <div style:background-color={color} />
    </div>
  {/each}
</div>

<style lang="scss">
  .legends {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;

    .lang {
      display: flex;
      gap: 5px;
      align-items: center;
      div:first-child {
        font-size: 14px;
        color: white;
      }

      div:last-child {
        border-radius: 16px;
        height: 12px;
        width: 12px;
      }
    }
  }
</style>
