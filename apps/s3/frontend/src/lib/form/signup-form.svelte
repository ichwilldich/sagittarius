<script lang="ts">
  import { Card } from "positron-components";
  import { Label } from "positron-components";
  import { Input } from "positron-components";
  import { Button } from "positron-components";
  import { cn } from "positron-components";
  import type { HTMLAttributes } from "svelte/elements";
  let { class: className, ...restProps }: HTMLAttributes<HTMLDivElement> = $props();
  const id = $props.id();
  import { CloudCheck, KeyRound, Database } from "@lucide/svelte";
</script>

<div class={cn("flex flex-col gap-6", className)} {...restProps}>
  <Card.Root class="overflow-hidden p-0">
    <Card.Content class="grid p-0 md:grid-cols-2">
      <form class="p-6 md:p-8">
        <div class="flex flex-col gap-6">
          <div class="flex flex-col items-center text-center">
            <h1 class="text-2xl font-bold">Create your account</h1>
            <p class="text-muted-foreground text-balance">
              Sign up to get started
            </p>
          </div>

          <div class="grid gap-3">
            <Label for="name-{id}">Full name</Label>
            <Input id="name-{id}" type="text" placeholder="Max Mustermann" required />
          </div>

          <div class="grid gap-3">
            <Label for="email-{id}">Email</Label>
            <Input id="email-{id}" type="email" placeholder="m@example.com" required />
          </div>

          <div class="grid gap-3">
            <Label for="password-{id}">Password</Label>
            <Input id="password-{id}" type="password" required />
          </div>

          <div class="grid gap-3">
            <Label for="confirm-{id}">Confirm Password</Label>
            <Input id="confirm-{id}" type="password" required />
          </div>

          <div class="flex items-center gap-2">
            <input id="tos-{id}" type="checkbox" class="h-4 w-4 rounded border-muted-foreground" required />
            <label for="tos-{id}" class="text-sm text-muted-foreground">
              I agree to the <a href="##" class="underline">Terms of Service</a> and <a href="##" class="underline">Privacy Policy</a>
            </label>
          </div>

          <Button type="submit" class="w-full">Create account</Button>

          <div class="text-center text-sm">
            Already have an account?
            <a href="/login" class="underline underline-offset-4"> Log in </a>
          </div>
        </div>
      </form>

      <div class="bg-muted relative hidden md:flex items-center justify-center p-8">
        <div class="flex flex-col items-center gap-3 text-center">
          <Database class="h-40 w-40 text-violet-500 dark:text-violet-400" aria-hidden="true" />
          <span class="text-lg font-semibold text-violet-600 dark:text-violet-300">ichwilldich</span>
        </div>
      </div>
    </Card.Content>
  </Card.Root>

  <div
    class="text-muted-foreground *:[a]:hover:text-primary *:[a]:underline *:[a]:underline-offset-4 text-balance text-center text-xs"
  >
    By creating an account, you agree to our <a href="##">Terms of Service</a> and
    <a href="##">Privacy Policy</a>.
  </div>
</div>
