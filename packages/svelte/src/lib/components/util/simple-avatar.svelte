<script lang="ts">
  import * as Avatar from '../ui/avatar/index.js';

  let { src, class: className }: { src: string; class: string } = $props();
</script>

<Avatar.Root class={className}>
  <Avatar.Image src={`data:image/webp;base64, ${src}`} alt="Avatar" />
  <Avatar.Fallback>?</Avatar.Fallback>
</Avatar.Root>
