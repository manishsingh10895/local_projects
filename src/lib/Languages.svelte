<script lang="ts">
    export let languages: Map<String, number>;
</script>

<div class="languages-line-container">
    <div class="languages">
        {#each Object.keys(languages) as lang}
            <span class="lang">
                {lang}
            </span>
        {/each}
    </div>
</div>