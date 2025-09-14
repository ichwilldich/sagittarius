<script lang="ts" generics="S extends FormRecord = FormRecord">
  import * as InputOtp from '../ui/input-otp/index.js';
  import * as Form from '../ui/form/index.js';
  import type { FormPath, SuperForm } from 'sveltekit-superforms';
  import type { FormRecord } from './types.js';

  interface Props {
    class: string | undefined;
    key: FormPath<S>;
    label: string;
    formData: SuperForm<S>;
    disabled?: boolean;
  }

  let {
    class: className,
    key,
    formData: form,
    label,
    disabled
  }: Props = $props();

  let { form: formData } = $derived(form);
</script>

<Form.Field {form} name={key}>
  <Form.Control>
    {#snippet children({ props })}
      <Form.Label class={className}>{label}</Form.Label>
      {/* @ts-ignore */ null}
      <InputOtp.Root
        maxlength={6}
        bind:value={$formData[key]}
        class={className}
        autofocus
        {disabled}
        {...props}
      >
        {#snippet children({ cells })}
          <InputOtp.Group>
            {#each cells.slice(0, 3) as cell}
              <InputOtp.Slot {cell} />
            {/each}
          </InputOtp.Group>
          <InputOtp.Separator />
          <InputOtp.Group>
            {#each cells.slice(3, 6) as cell}
              <InputOtp.Slot {cell} />
            {/each}
          </InputOtp.Group>
        {/snippet}
      </InputOtp.Root>
    {/snippet}
  </Form.Control>
  <Form.FieldErrors class={className} />
</Form.Field>
