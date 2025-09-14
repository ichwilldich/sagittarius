<script lang="ts">
  import ArrowUpDown from '@lucide/svelte/icons/arrow-up-down';
  import { Button } from '../ui/button/index.js';
  import type { ComponentProps } from 'svelte';

  let {
    variant = 'ghost',
    title,
    ...restProps
  }: ComponentProps<typeof Button> = $props();
</script>

<Button {variant} {...restProps}>
  {title}
  <ArrowUpDown class="ml-2 size-4" />
</Button>
