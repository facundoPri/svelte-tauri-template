<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { X } from '@lucide/svelte';

  let { children = undefined } = $props();

  const handleClose = async () => {
    await invoke("hide");
  };

</script>

<div
  class="flex items-center p-2 border-b select-none"
  data-tauri-drag-region
  role="toolbar"
  aria-label="Window controls"
  tabindex="0"
>
  <div class="flex">
    <button
      class="w-[12px] h-[12px] rounded-full bg-[#FF5F57] border border-[rgba(0,0,0,0.2)] flex items-center justify-center group"
      onclick={handleClose}
      aria-label="Close window"
    >
      <X class="opacity-0 group-hover:opacity-100" />
    </button>
  </div>

  <!-- Title content in the center -->  
   <div class="flex-1 flex items-center justify-center overflow-hidden h-full" data-tauri-drag-region>
    {#if children}
      {@render children()}
    {/if}
  </div>
</div>
