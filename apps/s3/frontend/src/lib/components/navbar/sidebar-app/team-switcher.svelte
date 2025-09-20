<script lang="ts">
  import { DropdownMenu } from "positron-components/components/ui";
  import { Sidebar } from "positron-components/components/ui";
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import { Database } from "@lucide/svelte";
    import { version } from "vite";
  // This should be `Component` after @lucide/svelte updates types
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let { teams }: { teams: { name: string; logo: any; plan: string }[] } = $props();
  let sidebar = Sidebar.useSidebar();
  let activeTeam = $state(teams[0]);
   const team = { name: "ichwilldich", logoLetter: "A", version: '0.0.1 beta'};

</script>

<div class="flex items-center gap-3 p-2">
  <div class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square w-8 h-8 items-center justify-center rounded-lg">
    <span class="text-sm font-semibold"><Database class="w-4 h-4" /></span>
  </div>
  <div class="grid">
    <span class="truncate font-medium text-sm">{team.name}</span>
    <span class="truncate text-xs text-muted-foreground">{team.version}</span>
  </div>
</div>