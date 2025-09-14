import type { superForm } from 'sveltekit-superforms';

export type { SuperForm, FormPath } from 'sveltekit-superforms';

export type FormRecord = Record<string, unknown>;

export interface FormSchema<
  T extends FormRecord = FormRecord,
  In extends FormRecord = T
> {
  schema: any;
  form: FormType<T, In>;
}

export type FormType<
  T extends FormRecord = FormRecord,
  In extends FormRecord = T
> = Parameters<typeof superForm<T, any, In>>[0];

export interface Error {
  field?: string;
  error: string;
}
