<script
  lang="ts"
  generics="CD, T, CT extends FormRecord = FormRecord, CIn extends FormRecord = CT, ET extends FormRecord = FormRecord, EIn extends FormRecord = ET, DT extends FormRecord = FormRecord, DIn extends FormRecord = DT"
>
  import type {
    Error,
    FormRecord,
    FormSchema,
    FormType
  } from '../form/types.js';
  import { createTable } from '../table/helpers.svelte';
  import Table from '../table/base-table.svelte';
  import { toast } from 'svelte-sonner';
  import type { ColumnDef, Row } from '@tanstack/table-core';
  import { RequestError } from '$lib/backend/types.svelte';
  import type { Snippet, SvelteComponent } from 'svelte';
  import type { SuperForm } from 'sveltekit-superforms';
  import FormDialog from '../form/form-dialog.svelte';

  interface Props {
    data: T[] | undefined;
    filter_keys: string[];
    columns: (
      editFn: (id: string) => void,
      deleteFn: (id: string) => void,
      data: CD
    ) => ColumnDef<T>[];
    label: string;
    createItemFn: (
      form: FormType<CT, CIn>
    ) => Promise<RequestError | undefined>;
    editItemFn: (item: T) => Promise<RequestError | undefined>;
    deleteItemFn: (id: string) => Promise<RequestError | undefined>;
    toId: (item: T) => string;
    display: (item: T | undefined) => string | undefined;
    title: string;
    description: string;
    editDialog?: Snippet<
      [
        {
          props: {
            disabled: boolean;
            formData: SuperForm<ET>;
          };
          item: T;
        }
      ]
    >;
    createDialog?: Snippet<
      [
        {
          props: {
            disabled: boolean;
            formData: SuperForm<CT>;
          };
        }
      ]
    >;
    createButtonDisabled: boolean;
    createForm: FormSchema<CT, CIn>;
    editForm: FormSchema<ET, EIn>;
    deleteForm: FormSchema<DT, DIn>;
    startCreate?: () => boolean | Promise<boolean>;
    startEdit?: (item: T) => void | Promise<void>;
    createClass?: string;
    editClass?: string;
    errorMappings?: {
      [key in RequestError]?: Error;
    };
    columnData: CD;
  }

  let {
    data,
    filter_keys,
    columns,
    label,
    createItemFn,
    editItemFn,
    deleteItemFn,
    toId,
    display,
    title,
    description,
    editDialog,
    createDialog,
    createButtonDisabled,
    createForm,
    editForm,
    deleteForm,
    startCreate,
    startEdit,
    createClass,
    editClass,
    errorMappings,
    columnData
  }: Props = $props();

  let isLoading = $state(false);
  let labelLower = $derived(label.toLocaleLowerCase());
  let fieldProps = $derived({ disabled: isLoading });

  let selected: T | undefined = $state();
  let editOpen = $state(false);
  let deleteOpen = $state(false);

  let editComp: SvelteComponent | undefined = $state();
  let setEditValue: (value: { [key: string]: string }) => void = $derived(
    editComp?.setValue || (() => {})
  );

  const filterFn = (row: Row<T>, id: string, filterValues: any) => {
    const info = filter_keys
      .map((k) => (row.original as any)[k] as string)
      .filter(Boolean)
      .join(' ')
      .toLowerCase();

    let searchTerms = Array.isArray(filterValues)
      ? filterValues
      : [filterValues];
    return searchTerms.some((term) => info.includes(term.toLowerCase()));
  };

  let table = $state(
    createTable(
      [],
      columns(
        () => {},
        () => {},
        columnData
      ),
      filterFn
    )
  );

  $effect(() => {
    table = createTable(
      data || [],
      columns(editItem, deleteItem, columnData),
      filterFn
    );
  });

  const createItem = async (form: FormType<CT, CIn>) => {
    let ret = await createItemFn(form);

    if (ret) {
      if (errorMappings && errorMappings[ret]) {
        return errorMappings[ret];
      } else {
        return { error: `Error while creating ${labelLower}` };
      }
    } else {
      toast.success(`Created ${label}`);
    }
  };

  const editItem = (id: string) => {
    selected = data?.find((item) => toId(item) === id);
    if (selected) {
      setEditValue(selected);
      startEdit?.(selected);
    }
    editOpen = true;
  };

  const deleteItem = (id: string) => {
    selected = data?.find((item) => toId(item) === id);
    deleteOpen = true;
  };

  const editItemConfirm = async (form: FormType<ET, EIn>) => {
    selected = {
      ...selected,
      ...(form.data as any)
    } as any;

    if (!selected) {
      return;
    }

    let ret = await editItemFn(selected);

    if (ret) {
      if (errorMappings && errorMappings[ret]) {
        return errorMappings[ret];
      } else {
        return { error: `Error while updating ${labelLower}` };
      }
    } else {
      toast.success(`${label} updated`);
    }
  };

  const deleteItemConfirm = async () => {
    if (!selected) {
      return;
    }

    let ret = await deleteItemFn(toId(selected));

    if (ret) {
      return { error: `Error while deleting ${label}` };
    } else {
      toast.success(`${label} deleted`);
    }
  };
</script>

<FormDialog
  title={`Delete ${label}`}
  description={`Do you really want to delete the ${labelLower} ${display(selected)}?`}
  confirm="Delete"
  confirmVariant="destructive"
  onsubmit={deleteItemConfirm}
  bind:open={deleteOpen}
  bind:isLoading
  form={deleteForm}
/>
<FormDialog
  bind:this={editComp}
  title={`Edit ${label}`}
  description={`Edit the ${labelLower} info for ${display(selected)} below`}
  confirm="Confirm"
  onsubmit={editItemConfirm}
  bind:open={editOpen}
  bind:isLoading
  form={editForm}
  class={editClass}
>
  {#snippet children({ props })}
    {#if selected}
      {@render editDialog?.({
        props: { ...props, ...fieldProps },
        item: selected
      })}
    {/if}
  {/snippet}
</FormDialog>
<div class="m-4 flex flex-1 flex-col space-y-3">
  <div class="ml-7 md:m-0">
    <h3 class="text-xl font-medium">{title}</h3>
    <p class="text-muted-foreground text-sm">
      {description}
    </p>
  </div>
  <Table filterColumn="name" {table} class="min-h-0 flex-1">
    <FormDialog
      title={`Create ${label}`}
      description={`Enter the details for the new ${labelLower} below`}
      confirm="Create"
      trigger={{
        text: `Create ${label}`,
        variant: 'secondary',
        class: 'ml-2',
        disabled: createButtonDisabled
      }}
      onsubmit={createItem}
      onopen={startCreate}
      bind:isLoading
      form={createForm}
      class={createClass}
    >
      {#snippet children({ props })}
        {@render createDialog?.({ props: { ...props, ...fieldProps } })}
      {/snippet}
    </FormDialog>
  </Table>
</div>
