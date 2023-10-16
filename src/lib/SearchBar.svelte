<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { Subscription, fromEvent } from "rxjs";
  import AiOutlineSearch from "svelte-icons-pack/ai/AiOutlineSearch";
  import { debounceTime, distinctUntilChanged, map, tap } from "rxjs/operators";
  import Icon from "svelte-icons-pack";

  let searchEl: HTMLInputElement;

  let subscription: Subscription;

  onMount(() => {
    let sub = fromEvent(searchEl, "input")
      .pipe(
        debounceTime(350),
        map((e: any) => e.target.value),
        distinctUntilChanged(),
        tap((value) => {
          console.log("[searchEl] pipe");
          console.log(value);
          onSearchChange(value);
        })
      )
      .subscribe();

    subscription = sub;
  });

  onDestroy(() => {
    subscription.unsubscribe();
  });

  export let onSearchChange: Function;

  onDestroy(() => {});
</script>

<div class="search-bar">
  <div class="search-bar__inner">
    <Icon color="gray" className="search-icon" src={AiOutlineSearch} />
    <input spellcheck="false" autocomplete="off" bind:this={searchEl} />
  </div>
</div>

<style lang="scss">
  :global(.search-icon) {
    position: absolute;
    top: 13px;
    left: 5px;
    color: gray;
  }

  .search-bar {
    height: 52px;
    display: flex;
    width: 100%;
    justify-content: center;

    &__inner {
      height: 100%;
      position: relative;
    }

    input {
      font-size: 18px;
      appearance: none;
      border-radius: 16px;
      width: 100%;
      height: 100%;
      max-width: 350px;
      border: 1px solid var(--primary-color);
      padding: 2px 18px;
      padding-left: 40px;
    }
    margin-bottom: 15px;
  }
</style>
