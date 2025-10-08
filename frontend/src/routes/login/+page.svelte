<script lang="ts">
  import { Button, Card, Switch } from 'positron-components/components/ui';
  import {
    BaseForm,
    FormInput,
    type FormType
  } from 'positron-components/components/form';
  import type { PageServerData } from './$types';
  import { loginSchema } from './schema.svelte';
  import { Database, LoaderCircle } from '@lucide/svelte';
  import { password_login } from '$lib/backend/auth.svelte';
  import { goto } from '$app/navigation';
  import { RequestError } from 'positron-components/backend';
  import {
    get_oidc_url,
    get_sso_config,
    SSOType,
    type SSOConfig
  } from '$lib/backend/sso.svelte';

  const { data }: { data: PageServerData } = $props();

  let sso_config: SSOConfig | undefined = $state();
  let sso_loading = $state(false);
  get_sso_config().then((config) => {
    sso_config = config;
  });

  const loginForm = {
    schema: loginSchema,
    form: data
  };

  const onsubmit = async (form: FormType<any>) => {
    let ret = await password_login(form.data.username, form.data.password);

    if (ret === RequestError.Unauthorized) {
      return { error: 'Invalid username or password.' };
    } else if (ret) {
      return { error: 'Login failed. Please try again later.' };
    } else {
      // wait for the next tick to ensure the session is updated
      setTimeout(() => {
        goto('/');
      });
    }
  };

  const sso_login = async () => {
    if (sso_loading) return;
    sso_loading = true;

    switch (sso_config?.sso_type) {
      case SSOType.Oidc:
        let url = await get_oidc_url();
        if (url) {
          window.location.href = url;
        }
        break;
      default:
        console.error('Unsupported SSO type or SSO not configured.');
        break;
    }

    sso_loading = false;
  };
</script>

<div class="bg-muted flex min-h-svh items-center justify-center p-6 md:p-10">
  <div class="max-w-sm md:max-w-3xl">
    <Card.Root class="overflow-hidden p-0">
      <Card.Content class="grid p-0 md:grid-cols-2">
        <BaseForm
          isLoading={false}
          {onsubmit}
          confirm="Login"
          form={loginForm}
          class="p-6 md:p-8"
        >
          {#snippet children({ props })}
            <div class="flex flex-col items-center text-center">
              <h1 class="text-2xl font-bold">Welcome back</h1>
              <p class="text-muted-foreground text-balance">
                Login to your account
              </p>
            </div>
            <FormInput
              key="username"
              label="Username"
              placeholder="John"
              autocapitalize="none"
              autocorrect="off"
              autocomplete="username"
              {...props}
            />
            <FormInput
              key="password"
              label="Password"
              placeholder="••••••••"
              type="password"
              autocomplete="current-password"
              autocapitalize="none"
              autocorrect="off"
              {...props}
            />
          {/snippet}
          {#snippet footer({ children })}
            <br />
            {@render children()}
            {#if sso_config && sso_config?.sso_type !== SSOType.None}
              <div
                class="after:border-border relative text-center text-sm after:absolute after:inset-0 after:top-1/2 after:z-0 after:flex after:items-center after:border-t"
              >
                <span class="bg-card text-muted-foreground relative z-10 px-2">
                  Or continue with
                </span>
              </div>

              <Button
                variant="outline"
                type="button"
                class="w-full"
                onclick={sso_login}
                disabled={sso_loading}
              >
                {#if sso_loading}
                  <LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
                {:else}
                  <Database class="mr-2 h-4 w-4" />
                {/if}
                SSO Login
              </Button>
            {/if}
          {/snippet}
        </BaseForm>
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
                Sagittarius
              </h2>
            </div>

            <p class="text-muted-foreground max-w-sm text-balance">
              S3 compatible cloud storage with modern authentication
            </p>
          </div>
        </div>
      </Card.Content>
    </Card.Root>
  </div>
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
