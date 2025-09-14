<script lang="ts">
  import CalendarIcon from '@lucide/svelte/icons/calendar';
  import {
    type DateValue,
    DateFormatter,
    getLocalTimeZone
  } from '@internationalized/date';
  import { cn } from '$lib/utils.js';
  import { Button } from '../ui/button/index.js';
  import { Calendar } from '../ui/calendar/index.js';
  import * as Popover from '../ui/popover/index.js';

  interface Props {
    value?: DateValue;
    class?: string;
    start?: DateValue;
    end?: DateValue;
  }

  let { value = $bindable(), class: className, start, end }: Props = $props();

  const df = new DateFormatter('en-US', {
    dateStyle: 'long'
  });
</script>

<Popover.Root>
  <Popover.Trigger>
    {#snippet child({ props })}
      <Button
        variant="outline"
        class={cn(
          'w-[280px] justify-start text-left font-normal',
          !value && 'text-muted-foreground',
          className
        )}
        {...props}
      >
        <CalendarIcon class="mr-2 size-4" />
        {value ? df.format(value.toDate(getLocalTimeZone())) : 'Select a date'}
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content class="w-auto p-0">
    <Calendar
      bind:value
      type="single"
      initialFocus
      minValue={start}
      maxValue={end}
      preventDeselect
    />
  </Popover.Content>
</Popover.Root>
