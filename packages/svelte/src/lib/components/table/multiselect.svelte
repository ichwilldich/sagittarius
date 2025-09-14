<script lang="ts" module>
  export interface Group<T> {
    label: string;
    items: Item<T>[];
  }

  export interface Item<T> {
    label: string;
    value: T;
  }
</script>

<script lang="ts">
  import * as Popover from '../ui/popover/index.js';
  import * as Command from '../ui/command/index.js';
  import { Button } from '../ui/button/index.js';
  import { cn } from '../../utils.js';
  import Check from '@lucide/svelte/icons/check';
  import { ScrollArea } from '../ui/scroll-area/index.js';

  type T = $$Generic;

  const isGroups = (object: any[]): object is Group<T>[] => {
    return (
      object.length > 0 &&
      typeof object[0] === 'object' &&
      object[0] !== null &&
      'items' in object[0]
    );
  };

  interface Props {
    data: Group<T>[] | Item<T>[];
    filter?: (data: Item<T>) => boolean;
    label: string;
    compare?: (a: T, b: T) => boolean;
    disabled?: boolean;
    single?: boolean;
    selected: T[];
    buttonPrefix?: string;
    class?: string;
  }

  let {
    data,
    filter = () => true,
    label,
    compare = (a, b) => a === b,
    disabled,
    single,
    selected = $bindable([]),
    buttonPrefix,
    class: className
  }: Props = $props();

  let open = $state(false);

  const select = (value: T) => {
    if (single) {
      selected = [value];
      open = false;
    } else {
      let index = selected.findIndex((i) => compare(i, value));
      if (index !== -1) {
        selected.splice(index, 1);
      } else {
        selected.push(value);
      }
    }
  };

  let filtered = $derived.by(() => {
    if (isGroups(data)) {
      return data
        .map((g) => {
          g.items = g.items.filter(filter);
          return g;
        })
        .filter((g) => g.items.length > 0);
    } else {
      return [
        {
          label: '',
          items: data.filter(filter)
        }
      ];
    }
  });

  const find_element = (value: T): Item<T> | undefined => {
    return filtered
      .map(
        (g) =>
          g.items
            .map((i) => (compare(i.value, value) ? i : undefined))
            .filter((i) => i !== undefined)[0]
      )
      .filter((i) => i !== undefined)[0];
  };
</script>

<Popover.Root bind:open>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button
        variant="outline"
        {...props}
        role="combobox"
        class={cn('block truncate', className)}
        {disabled}
      >
        {#if selected.length === 0}
          {buttonPrefix ?? 'No'} {label}
        {:else}
          {selected.map((s) => find_element(s)?.label).join(', ')}
        {/if}
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content>
    <Command.Root>
      <Command.Input placeholder={`Search ${label.toLowerCase()}...`} />
      <Command.List class="h-full overflow-hidden">
        <ScrollArea class="max-h-[300px] overflow-y-auto">
          <Command.Empty>No {label} found</Command.Empty>
          {#each filtered as group}
            <Command.Group heading={group.label}>
              {#each group.items as item}
                <Command.Item
                  value={item.label}
                  onSelect={() => select(item.value)}
                >
                  <Check
                    class={cn(
                      'mr-2 size-4',
                      !selected.some((i) => compare(i, item.value)) &&
                        'text-transparent'
                    )}
                  />
                  {item.label}
                </Command.Item>
              {/each}
            </Command.Group>
          {/each}
        </ScrollArea>
      </Command.List>
    </Command.Root>
  </Popover.Content>
</Popover.Root>
