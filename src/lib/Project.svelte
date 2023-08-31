<style lang="scss">
    .project {
        border-radius: 16px;
        margin: 10px 0;
        padding: 15px 10px;
        .name {
            font-size: 1.8rem;
            font-weight: bold;
        }
    }
</style>

<div class="project">
    <div class="name">
        {project.name}
    </div>
</div>

<script lang="ts">
    export let project;
</script>