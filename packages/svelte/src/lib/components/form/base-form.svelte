<script
  lang="ts"
  generics="T extends FormRecord = FormRecord, In extends FormRecord = T"
>
  import type { Snippet } from 'svelte';
  import { get } from 'svelte/store';
  import { setError, superForm, type SuperForm } from 'sveltekit-superforms';
  import { zodClient } from 'sveltekit-superforms/adapters';
  import { FormButton } from '../ui/form/index.js';
  import LoaderCircle from '@lucide/svelte/icons/loader-circle';
  import type { ButtonVariant } from '../ui/button/index.js';
  import { cn } from '../../utils.js';
  import type { Error, FormRecord, FormSchema, FormType } from './types.js';

  interface Props {
    form: FormSchema<T, In>;
    onsubmit: (
      form: FormType<T, In>
    ) => Error | undefined | Promise<Error | undefined>;
    children?: Snippet<
      [{ props: { formData: SuperForm<T>; disabled: boolean } }]
    >;
    footer: Snippet<[{ children: Snippet<[{ className?: string }?]> }]>;
    isLoading: boolean;
    confirmVariant?: ButtonVariant;
    confirm: string;
    error?: string;
    class?: string;
  }

  let {
    form: formInfo,
    onsubmit,
    children,
    footer,
    isLoading = $bindable(false),
    confirmVariant = 'default',
    confirm,
    error = $bindable(''),
    class: className
  }: Props = $props();

  let form = superForm(formInfo.form, {
    validators: zodClient(formInfo.schema),
    SPA: true,
    onUpdate: async ({ form, cancel }) => {
      if (!form.valid) return;

      error = '';
      isLoading = true;

      let ret = await onsubmit(form);

      isLoading = false;
      if (ret) {
        if (ret.field) {
          setError(form, ret.field as '', ret.error, undefined);
        } else {
          if (ret.error !== '') error = ret.error;
          cancel();
        }
      }
    }
  });

  let { enhance } = form;

  export const setValue = (value: T) => {
    let old = get(form.form);

    let newValue: T = {} as any;
    for (const key in old) {
      newValue[key] = value[key] ?? old[key];
    }

    form.form.set(newValue);
  };
</script>

<form method="POST" class={cn('grid gap-3', className)} use:enhance>
  {@render children?.({ props: { formData: form, disabled: isLoading } })}
  {#if error}
    <span class="text-destructive truncate text-sm">{error}</span>
  {/if}
  {@render footer({ children: formButton })}
</form>

{#snippet formButton(props: { className?: string } | undefined)}
  {@const prop = { ...props }}
  <FormButton
    class={prop.className}
    type="submit"
    disabled={isLoading}
    variant={confirmVariant}
  >
    {#if isLoading}
      <LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
    {/if}
    {confirm}
  </FormButton>
{/snippet}
