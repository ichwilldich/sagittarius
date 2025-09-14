import { tick, untrack } from 'svelte';

const createKeyedWatcher = () => {
  let watchers = new Map();

  return {
    watch(setup: () => () => void) {
      if ($effect.tracking()) {
        $effect(() => {
          let entry = watchers.get(setup);
          if (!entry) {
            const cleanup = untrack(setup);
            entry = [0, cleanup];
            watchers.set(setup, entry);
          }
          entry[0]++;

          return () => {
            tick().then(() => {
              entry[0]--;
              if (entry[0] === 0) {
                entry[1]?.();
                watchers.delete(setup);
              }
            });
          };
        });
      }
    }
  };
};

export const interval = <T>(update: () => T, timeout: number) => {
  let value = $state(update());

  const setup = () => {
    const id = setInterval(() => {
      value = update();
    }, timeout);
    return () => clearInterval(id);
  };

  const watcher = createKeyedWatcher();

  return {
    get value() {
      watcher.watch(setup);
      return value;
    }
  };
};

export const wait_for = (
  condition: () => boolean,
  intervalTime = 100,
  max?: number
) => {
  return new Promise<boolean>((resolve) => {
    if (condition()) {
      resolve(true);
      return;
    }

    let timeout: undefined | number = undefined;
    const interval = setInterval(() => {
      if (condition()) {
        clearInterval(interval);
        if (timeout) {
          clearTimeout(timeout);
        }
        resolve(true);
      }
    }, intervalTime);

    if (max) {
      timeout = setTimeout(() => {
        clearInterval(interval);
        if (timeout) {
          clearTimeout(timeout);
        }
        resolve(false);
      }, max);
    }
  });
};

export const sleep = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};
