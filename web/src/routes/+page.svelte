<script lang="ts">
  import { check_shortcut } from "$lib/helpers";
  import { flip } from "svelte/animate";
  import { fly } from "svelte/transition";
  // Has to be dynamically imported for prerendering to work
  // https://github.com/sveltejs/svelte/issues/13155
  const cpc_promise = import("cpc");
  let cpc: typeof import("cpc") | undefined;
  cpc_promise.then((mod) => {
    cpc = mod;
  });

  let input = $state("");
  let output = $derived.by(() => {
    try {
      if (!cpc || input.trim().length === 0) {
        return "";
      }
      return cpc.wasm_eval(input);
    } catch (e) {
      return "";
    }
  });
  let saved_queries: { id: number; in: string; out: string }[] = $state([]);
</script>

<svelte:head>
  <title>cpc</title>
  <meta
    name="description"
    content="Text calculator with support for units and conversion"
  />
</svelte:head>

<main class="w-full px-4 lg:px-8 text-base lg:text-lg">
  <nav class="flex items-center justify-between py-4 lg:py-6">
    <h1 class="text-3xl font-bold text-amber-600 dark:text-amber-400">cpc</h1>
    <a
      href="https://github.com/probablykasper/cpc"
      aria-label="GitHub repository"
      class="svelte-1ugh5mt"
    >
      <svg
        height="24"
        viewBox="-2 -2 28 28"
        width="24"
        xmlns="http://www.w3.org/2000/svg"
        class="svelte-8lfi33 svelte-1ugh5mt"
        ><path
          d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"
          fill="none"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          class="svelte-8lfi33"
        ></path></svg
      >
    </a>
  </nav>
  <input
    type="text"
    class="border border-gray-500/50 w-full rounded-lg px-3 py-2 outline-none"
    bind:value={input}
    onkeydown={(e) => {
      if (check_shortcut(e, "Enter")) {
        const input = e.currentTarget.value;
        let out;
        try {
          out = wasm_eval(e.currentTarget.value);
        } catch (e) {
          out = "";
        }
        console.log(out);
        saved_queries.unshift({
          id: saved_queries.length,
          in: input,
          out,
        });
        e.currentTarget.value = "";
        output = "";
      }
    }}
    placeholder="10km/h * 1 decade in light seconds"
  />
  <div class="pt-1 leading-tight">
    <div class="px-3 py-2">
      {output}<span class="invisible select-none">x</span>
    </div>
    {#each saved_queries as query (query.id)}
      <div
        class="px-3 py-2"
        in:fly={{ y: -10, duration: 150 }}
        animate:flip={{ duration: 150 }}
      >
        <p class="opacity-65 text-base leading-tight">{query.in}</p>
        <p>{query.out}</p>
      </div>
    {/each}
  </div>
</main>

<style lang="postcss">
  @reference "../app.css";

  @media (prefers-color-scheme: dark) {
    :global(body) {
      @apply bg-black text-white;
    }
  }
</style>
