<script lang="ts">
  import '../app.css';
  import AppSidebar from '$lib/components/navbar/sidebar-app/sidebar-app.svelte';
  import { ModeWatcher, Sidebar } from 'positron-components/components/ui';
  import { page } from '$app/state';

  interface Props {
    children?: import('svelte').Snippet;
  }

  let { children }: Props = $props();

  const noLayout = ['/login', '/oauth', '/oauth/logout'];
  $inspect(page.url.pathname).with(console.log);
</script>

<ModeWatcher />

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
