<script lang="ts">
  import * as Table from '../ui/table/index.js';
  import * as Dropdown from '../ui/dropdown-menu/index.js';
  import { FlexRender } from '../ui/data-table/index.js';
  import { Input } from '../ui/input/index.js';
  import { ScrollArea } from '../ui/scroll-area/index.js';
  import type { Table as TableType } from '@tanstack/table-core';
  import { Button } from '../ui/button/index.js';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import type { Snippet } from 'svelte';
  import { cn } from '../../utils.js';

  type T = $$Generic;

  interface Props {
    table: TableType<T>;
    children?: Snippet;
    filterColumn: string;
    class?: string;
  }

  let { table, children, filterColumn, class: className }: Props = $props();
</script>

<div class={cn('flex w-full flex-col', className)}>
  <div class="flex items-center py-4">
    <Input
      placeholder="Filter entries"
      value={(table.getColumn(filterColumn)?.getFilterValue() as string) ?? ''}
      oninput={(e) =>
        table.getColumn(filterColumn)?.setFilterValue(e.currentTarget.value)}
      onchange={(e) =>
        table.getColumn(filterColumn)?.setFilterValue(e.currentTarget.value)}
      class="mr-2 max-w-full"
    />
    <Dropdown.Root>
      <Dropdown.Trigger>
        {#snippet child({ props })}
          <Button {...props} variant="outline">
            Columns
            <ChevronDown class="ml-2 size-4" />
          </Button>
        {/snippet}
      </Dropdown.Trigger>
      <Dropdown.Content align="end">
        {#each table
          .getAllColumns()
          .filter((col) => col.getCanHide()) as column}
          <Dropdown.CheckboxItem
            class="capitalize"
            bind:checked={() => column.getIsVisible(), () => {}}
            onCheckedChange={(value) => column.toggleVisibility(!!value)}
          >
            {column.id}
          </Dropdown.CheckboxItem>
        {/each}
      </Dropdown.Content>
    </Dropdown.Root>
    {@render children?.()}
  </div>
  <ScrollArea class="grid min-h-0 flex-1 rounded-md border" orientation="both">
    <Table.Root
      class={`min-w-[${table.getHeaderGroups()[0].headers.length * 100}px]`}
    >
      <Table.Header>
        {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
          <Table.Row>
            {#each headerGroup.headers as header (header.id)}
              <Table.Head>
                {#if !header.isPlaceholder}
                  <FlexRender
                    content={header.column.columnDef.header}
                    context={header.getContext()}
                  />
                {/if}
              </Table.Head>
            {/each}
          </Table.Row>
        {/each}
      </Table.Header>
      <Table.Body>
        {#each table.getRowModel().rows as row (row.id)}
          <Table.Row data-state={row.getIsSelected() && 'selected'}>
            {#each row.getVisibleCells() as cell (cell.id)}
              <Table.Cell class="group">
                <div
                  class="last-group:justify-end last-group:text-center last-group:h-full group-last:flex"
                >
                  <FlexRender
                    content={cell.column.columnDef.cell}
                    context={cell.getContext()}
                  />
                </div>
              </Table.Cell>
            {/each}
          </Table.Row>
        {:else}
          <Table.Row>
            <Table.Cell
              colspan={table.getAllColumns().length}
              class="h-24 text-center"
            >
              No results.
            </Table.Cell>
          </Table.Row>
        {/each}
      </Table.Body>
    </Table.Root>
  </ScrollArea>
</div>
