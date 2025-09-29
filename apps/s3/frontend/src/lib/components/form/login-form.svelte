<script lang="ts">
  import { Card } from 'positron-components';
  import { Label } from 'positron-components';
  import { Input } from 'positron-components';
  import { Button } from 'positron-components';
  import { cn } from 'positron-components';
  import type { HTMLAttributes } from 'svelte/elements';
  import { CloudCheck, KeyRound, Database } from '@lucide/svelte';
  import { createEventDispatcher } from 'svelte';
  import { goto } from '$app/navigation';

  // Svelte 5: $props() nur einmal
  const props = $props() as HTMLAttributes<HTMLDivElement> & { baseUrl?: string; id?: string };
  let { class: className, id, baseUrl: baseUrlProp, ...restProps } = props;

  // Base-URL aus Prop, Env oder Fallback
  const baseUrl: string =
    (baseUrlProp as string | undefined) ??
    (import.meta.env.VITE_API_URL as string) ??
    '/backend';

  // Login-Route
  const endpoint = `${baseUrl}/auth`;

  const dispatch = createEventDispatcher();

  let email = $state('');
  let password = $state('');
  let loading = $state(false);
  let error: string | null = $state(null);

  async function submitForm(e: Event) {
    e.preventDefault();
    error = null;
    loading = true;
  
    try {
      const payload = { email, password };
      console.log('Sending login data:', payload); // Debug-Output
  
      const res = await fetch(endpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
      });
  
      console.log('Response status:', res.status); // Debug-Output
  
      if (res.status === 401) {
        error = 'Ungültige Zugangsdaten';
        dispatch('error', { message: error });
        return;
      }
  
      if (res.status === 422) {
        const json = await res.json().catch(() => null);
        error = json?.message ?? 'Ungültige Eingabedaten';
        console.log('422 Error details:', json); // Debug-Output
        dispatch('error', { message: error });
        return;
      }
  
      if (res.status === 204) {
        dispatch('success', { ok: true });
        goto('/');
        return;
      }
  
      if (!res.ok) {
        const json = await res.json().catch(() => null);
        error = json?.message ?? res.statusText;
        console.log('Error response:', json); // Debug-Output
        dispatch('error', { message: error });
        return;
      }
  
      const data = await res.json().catch(() => ({}));
      console.log('Login successful:', data);
      if (data?.token) {
        localStorage.setItem('authToken', data.token);
      }
  
      dispatch('success', data);
      goto('/');
    } catch (err) {
      console.error('Login error:', err); // Debug-Output
      error = (err as Error).message;
      dispatch('error', { message: error });
    } finally {
      loading = false;
    }
  }
</script>

<div class={cn('flex flex-col gap-6', className)} {...restProps}>
  <Card.Root class="overflow-hidden p-0">
    <Card.Content class="grid p-0 md:grid-cols-2">
      <form class="p-6 md:p-8" on:submit|preventDefault={submitForm}>
        <div class="flex flex-col gap-6">
          <div class="flex flex-col items-center text-center">
            <h1 class="text-2xl font-bold">Welcome back</h1>
            <p class="text-muted-foreground text-balance">
              Login to your account
            </p>
          </div>

          {#if error}
            <div class="text-sm text-red-600">{error}</div>
          {/if}

          <div class="grid gap-3">
            <Label for="email-{id}">Email</Label>
            <Input
              id="email-{id}"
              type="username"
              placeholder="deine@email.com"
              required
              bind:value={email}
              disabled={loading}
            />
          </div>
          <div class="grid gap-3">
            <div class="flex items-center">
              <Label for="password-{id}">Password</Label>
              <a
                href="##"
                class="ml-auto text-sm underline-offset-2 hover:underline"
              >
                Forgot your password?
              </a>
            </div>
            <Input
              id="password-{id}"
              type="password"
              required
              bind:value={password}
              disabled={loading}
            />
          </div>
          <Button type="submit" class="w-full" disabled={loading}>
            {#if loading}Loading...{:else}Login{/if}
          </Button>
        </div>
      </form>
    </Card.Content>
  </Card.Root>
</div>