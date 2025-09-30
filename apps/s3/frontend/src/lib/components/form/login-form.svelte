<script lang="ts">
  import { Card } from 'positron-components';
  import { Label } from 'positron-components';
  import { Input } from 'positron-components';
  import { Button } from 'positron-components';
  import { cn } from 'positron-components';
  import type { HTMLAttributes } from 'svelte/elements';
  import { Database } from '@lucide/svelte';
  import { createEventDispatcher } from 'svelte';
  import { goto } from '$app/navigation';
  import { password_login } from '$lib/backend/auth.svelte';

  // Svelte 5: $props() nur einmal
  const props = $props() as HTMLAttributes<HTMLDivElement> & {
    baseUrl?: string;
    id?: string;
  };
  let { class: className, id, baseUrl: baseUrlProp, ...restProps } = props;

  // Base-URL aus Prop, Env oder Fallback
  const baseUrl: string =
    (baseUrlProp as string | undefined) ??
    (import.meta.env.VITE_API_URL as string) ??
    '/backend';
  // Login-Route
  const endpoint = `${baseUrl}/auth`;

  const dispatch = createEventDispatcher();

  let name = $state('');
  let password = $state('');
  let loading = $state(false);
  let error: string | null = $state(null);

  async function submitForm(e: Event) {
    e.preventDefault();
    error = null;
    loading = true;

    try {
      const payload = { name: name, password };
      console.log('Sending login data:', payload);

      const res = await password_login(name, password);

      if (res) {
        error = 'Login failed. Please check your credentials.';
        dispatch('error', { message: error });
      } else {
        console.log('Login successful, redirecting...');
        goto('/');
      }
    } catch (err) {
      console.error('Login error:', err);
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
            <Label for="name-{id}">Name</Label>
            <Input
              id="name-{id}"
              type="text"
              placeholder="John"
              required
              bind:value={name}
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

          <div
            class="after:border-border relative text-center text-sm after:absolute after:inset-0 after:top-1/2 after:z-0 after:flex after:items-center after:border-t"
          >
            <span class="bg-card text-muted-foreground relative z-10 px-2">
              Or continue with
            </span>
          </div>

          <Button variant="outline" type="button" class="w-full">
            <Database class="mr-2 h-4 w-4" />
            SSO Login
          </Button>
        </div>
      </form>

      <div class="relative hidden overflow-hidden md:block">
        <!-- Blur Background -->
        <div
          class="absolute inset-0 bg-gradient-to-br from-purple-500/20 via-violet-500/10 to-indigo-500/20 backdrop-blur-sm"
        ></div>

        <!-- Dynamic Curves -->
        <div class="dynamic-curves">
          <div class="curve"></div>
          <div class="curve"></div>
          <div class="curve"></div>
        </div>

        <!-- Content über dem Blur -->
        <div
          class="relative z-10 flex h-full flex-col items-center justify-center p-8 text-center"
        >
          <div class="mb-6 flex items-center gap-3">
            <Database class="h-8 w-8 text-purple-600" />
            <h2
              class="bg-gradient-to-r from-purple-600 to-violet-600 bg-clip-text text-3xl font-bold text-transparent"
            >
              ichwilldich
            </h2>
          </div>

          <p class="text-muted-foreground max-w-sm text-balance">
            Secure cloud storage with advanced S3 compatibility and modern
            authentication
          </p>
        </div>
      </div>
    </Card.Content>
  </Card.Root>
</div>

<style>
  .dynamic-curves {
    position: absolute;
    inset: 0;
    opacity: 0.6;
  }

  .curve {
    position: absolute;
    border-radius: 50%;
    background: linear-gradient(
      135deg,
      rgba(139, 92, 246, 0.3) 0%,
      rgba(168, 85, 247, 0.2) 50%,
      rgba(147, 51, 234, 0.1) 100%
    );
    animation: float 6s ease-in-out infinite;
  }

  .curve:nth-child(1) {
    width: 200px;
    height: 200px;
    top: 10%;
    right: 20%;
    animation-delay: 0s;
  }

  .curve:nth-child(2) {
    width: 150px;
    height: 150px;
    bottom: 20%;
    left: 15%;
    animation-delay: 2s;
  }

  .curve:nth-child(3) {
    width: 100px;
    height: 100px;
    top: 50%;
    right: 10%;
    animation-delay: 4s;
  }

  @keyframes float {
    0%,
    100% {
      transform: translateY(0px) rotate(0deg);
    }
    50% {
      transform: translateY(-20px) rotate(180deg);
    }
  }
</style>
