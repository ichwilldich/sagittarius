<script lang="ts" generics="S extends FormRecord = FormRecord">
  import * as Form from '../ui/form/index.js';
  import { type FormPath, type SuperForm } from 'sveltekit-superforms';
  import { Input } from '../ui/input/index.js';
  import type {
    HTMLInputAttributes,
    HTMLInputTypeAttribute
  } from 'svelte/elements';
  import type { WithElementRef } from 'bits-ui';
  import type { FormRecord } from './types.js';

  type InputType = Exclude<HTMLInputTypeAttribute, 'file'>;

  type InputProps = WithElementRef<
    Omit<HTMLInputAttributes, 'type'> &
      (
        | { type: 'file'; files?: FileList }
        | { type?: InputType; files?: undefined }
      )
  >;

  interface Props {
    formData: SuperForm<S>;
    key: FormPath<S>;
    label: string;
    disabled?: boolean;
  }

  let {
    formData: form,
    key,
    label,
    disabled,
    ...restProps
  }: InputProps & Props = $props();

  const { form: formData } = $derived(form);
</script>

<Form.Field {form} name={key} class="gap-1/2 grid">
  <Form.Control>
    {#snippet children({ props })}
      <Form.Label>{label}</Form.Label>
      <Input {disabled} {...props} {...restProps} bind:value={$formData[key]} />
    {/snippet}
  </Form.Control>
  <Form.FieldErrors />
</Form.Field>
