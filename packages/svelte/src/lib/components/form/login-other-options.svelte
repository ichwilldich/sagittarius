<script lang="ts">
  import KeyRound from '@lucide/svelte/icons/key-round';
  import LoaderCircle from '@lucide/svelte/icons/loader-circle';
  import { Button } from '../ui/button/index.js';

  interface Props {
    isLoading: boolean;
    passkeyClick: () => void;
    passkeyError: string;
  }

  let { isLoading, passkeyClick, passkeyError }: Props = $props();
</script>

<div class="relative">
  <div class="absolute inset-0 flex items-center">
    <span class="w-full border-t"></span>
  </div>
  <div class="relative flex justify-center text-xs uppercase">
    <span class="bg-background text-muted-foreground px-2"
      >Or continue with
    </span>
  </div>
</div>
{#if passkeyError !== ''}
  <span class="text-destructive truncate text-sm">{passkeyError}</span>
{/if}
<Button
  variant="outline"
  type="button"
  disabled={isLoading}
  onclick={passkeyClick}
>
  {#if isLoading}
    <LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
  {:else}
    <KeyRound class="mr-2 h-4 w-4" />
  {/if}
  Passkey
</Button>
