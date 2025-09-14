<script lang="ts" generics="S extends FormRecord = FormRecord">
  import { Switch } from '../ui/switch/index.js';
  import * as Form from '../ui/form/index.js';
  import type { FormPath, SuperForm } from 'sveltekit-superforms';
  import type { FormRecord } from './types.js';

  interface Props {
    formData: SuperForm<S>;
    key: FormPath<S>;
    label: string;
    disabled?: boolean;
  }

  let { formData: form, key, label, disabled, ...restProps }: Props = $props();

  const { form: formData } = $derived(form as any);
</script>

<Form.Field {form} name={key} class="mt-2 flex w-full items-center">
  <Form.Control>
    {#snippet children({ props })}
      <Form.Label>{label}</Form.Label>
      <Switch
        {...props}
        {...restProps}
        bind:checked={$formData[key]}
        class="ml-auto"
      />
    {/snippet}
  </Form.Control>
  <Form.FieldErrors />
</Form.Field>
