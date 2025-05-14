<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import PanelTitleBar from "$lib/components/panel/PanelTitleBar.svelte";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    greetMsg = await invoke("greet", { name });
  }

  function focusInput() {
    // Focus the input element when the panel becomes visible
    setTimeout(() => {
      const inputElement = document.getElementById('greet-input') as HTMLInputElement;
      if (inputElement) {
        inputElement.focus();
      }
    }, 100); // Small delay to ensure the panel is fully visible
  }

  onMount(() => {
    // Listen for the panel_did_become_key event
    let unlistenFn: () => void;

    listen("panel_panel_did_become_key", () => {
      focusInput();
    }).then(unlisten => {
      unlistenFn = unlisten;
    });

    // Also focus on initial mount
    focusInput();

    return () => {
      if (unlistenFn) unlistenFn();
    };
  });
</script>

<div class="flex flex-col h-screen w-full overflow-hidden">
  <PanelTitleBar />

  <main class="flex-1 overflow-auto p-4 flex flex-col items-center justify-center">
    <h1 class="mb-4 text-2xl font-bold">Panel Window</h1>

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
</div>
