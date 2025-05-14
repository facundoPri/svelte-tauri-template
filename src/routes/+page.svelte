<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="container mx-auto flex min-h-screen flex-col items-center justify-center px-4 py-16">
  <h1 class="mb-8 text-4xl font-bold">Welcome to Tauri + Svelte</h1>

  <div class="mb-8 flex items-center justify-center space-x-6">
    <a href="https://vitejs.dev" target="_blank" class="transition-transform hover:scale-110">
      <img src="/vite.svg" class="h-16 w-16" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank" class="transition-transform hover:scale-110">
      <img src="/tauri.svg" class="h-16 w-16" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank" class="transition-transform hover:scale-110">
      <img src="/svelte.svg" class="h-16 w-16" alt="SvelteKit Logo" />
    </a>
  </div>

  <p class="mb-8 text-center text-lg text-muted-foreground">
    Click on the Tauri, Vite, and SvelteKit logos to learn more.
  </p>

  <form class="mb-6 flex w-full max-w-sm items-center gap-2" onsubmit={greet}>
    <Input
      id="greet-input"
      placeholder="Enter a name..."
      bind:value={name}
    />
    <Button type="submit">Greet</Button>
  </form>

  {#if greetMsg}
    <p class="text-center text-lg">{greetMsg}</p>
  {/if}
</main>
