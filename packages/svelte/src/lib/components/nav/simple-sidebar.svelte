<script lang="ts">
  import { page } from '$app/state';
  import { cn } from '../../utils.js';
  import { crossfade } from 'svelte/transition';
  import Button from '../ui/button/button.svelte';
  import { cubicInOut } from 'svelte/easing';

  interface Props {
    class: string | undefined;
    items: { href: string; title: string }[];
  }

  const { class: className = undefined, items }: Props = $props();

  const [send, receive] = crossfade({
    duration: 250,
    easing: cubicInOut
  });
</script>

<nav
  class={cn('flex space-x-2 lg:flex-col lg:space-y-1 lg:space-x-0', className)}
>
  {#each items as item}
    {@const isActive = page.url.pathname + page.url.search === item.href}

    <Button
      href={item.href}
      variant="ghost"
      class={cn(
        !isActive && 'hover:underline',
        'relative justify-start hover:bg-transparent'
      )}
      data-sveltekit-noscroll
    >
      {#if isActive}
        <div
          class="bg-muted absolute inset-0 rounded-md"
          in:send={{ key: 'active-sidebar-tab' }}
          out:receive={{ key: 'active-sidebar-tab' }}
        ></div>
      {/if}
      <div class="relative">
        {item.title}
      </div>
    </Button>
  {/each}
</nav>
