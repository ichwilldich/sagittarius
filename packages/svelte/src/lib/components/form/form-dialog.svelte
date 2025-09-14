<script
  lang="ts"
  generics="T extends FormRecord = FormRecord, In extends FormRecord = T"
>
  import * as Dialog from '../ui/dialog/index.js';
  import type { Snippet, SvelteComponent } from 'svelte';
  import {
    Button,
    type ButtonSize,
    type ButtonVariant
  } from '../ui/button/index.js';
  import LoaderCircle from '@lucide/svelte/icons/loader-circle';
  import { type SuperForm } from 'sveltekit-superforms';
  import type { Error, FormSchema, FormRecord, FormType } from './types.js';
  import Form from './base-form.svelte';
  import { wait_for } from '$lib/util/interval.svelte';

  interface Props {
    title: string;
    description?: string;
    confirm: string;
    confirmVariant?: ButtonVariant;
    open?: boolean;
    class?: string;
    isLoading?: boolean;
    trigger?: {
      text?: string;
      variant?: ButtonVariant;
      class?: string;
      size?: ButtonSize;
      loadIcon?: boolean;
      disabled?: boolean;
    };
    onopen?: () => boolean | Promise<boolean>;
    onsubmit: (
      form: FormType<T, In>
    ) => Error | undefined | Promise<Error | undefined>;
    children?: Snippet<
      [{ props: { formData: SuperForm<T>; disabled: boolean } }]
    >;
    triggerInner?: Snippet;
    form: FormSchema<T, In>;
  }

  let {
    title,
    description = '',
    confirm,
    confirmVariant = 'default',
    open = $bindable(false),
    class: className,
    trigger,
    isLoading = $bindable(false),
    onopen = () => true,
    onsubmit,
    children,
    triggerInner,
    form: formInfo
  }: Props = $props();

  let formComp: SvelteComponent | undefined = $state();
  let error = $state('');
  let formSetValue: undefined | ((value: T) => void) = $derived(
    formComp?.setValue
  );

  const submit = async (form: FormType<T, In>) => {
    let ret = await onsubmit(form);
    if (!ret) {
      open = false;
    }
    return ret;
  };

  export const openFn = async () => {
    isLoading = true;
    if (await onopen()) {
      open = true;
      error = '';
    }
    isLoading = false;
  };

  export const setValue = async (value: T) => {
    if (await wait_for(() => formSetValue !== undefined, 10, 5000)) {
      formSetValue!(value);
    }
  };
</script>

{#if trigger}
  <Button
    variant={trigger.variant}
    onclick={openFn}
    class={trigger.class}
    size={trigger.size}
    disabled={isLoading || trigger.disabled}
  >
    {#if isLoading && trigger.loadIcon}
      <LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
    {/if}
    {trigger.text}
    {@render triggerInner?.()}
  </Button>
{/if}
<Dialog.Root bind:open>
  <Dialog.Content class={className}>
    <Dialog.Header>
      <Dialog.Title>{title}</Dialog.Title>
      <Dialog.Description>{description}</Dialog.Description>
    </Dialog.Header>
    <Form
      bind:this={formComp}
      bind:isLoading
      bind:error
      form={formInfo}
      {confirmVariant}
      onsubmit={submit}
      {confirm}
      {children}
    >
      {#snippet footer({ children })}
        <Dialog.Footer class="mt-4">
          {@render children()}
        </Dialog.Footer>
      {/snippet}
    </Form>
  </Dialog.Content>
</Dialog.Root>
