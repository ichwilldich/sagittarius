<script lang="ts">
  import '../app.css';
  import AppSidebar from '$lib/components/navbar/sidebar-app/sidebar-app.svelte';
  import * as Sidebar from 'positron-components/components/ui/sidebar';
  import { ModeWatcher } from 'positron-components/components/util/general';
  import { Toaster } from 'positron-components/components/ui/sonner';
  import { page } from '$app/state';

  let { children } = $props();

  const noLayout = ['/login', '/oauth', '/oauth/logout'];
</script>

<ModeWatcher />
<Toaster position="top-right" richColors closeButton />

{#if !noLayout.includes(page.url.pathname)}
  <Sidebar.Provider class="min-h-screen">
    <AppSidebar />
    <Sidebar.Trigger class="absolute top-3 left-3 flex md:hidden" />
    <main class="min-h-screen min-w-0 flex-1">
      <div class="w-full">
        {@render children?.()}
      </div>
    </main>
  </Sidebar.Provider>
{:else}
  {@render children?.()}
{/if}
