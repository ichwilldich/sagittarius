<script lang="ts">
    import { onMount } from 'svelte';
    import { Search, Folder, Plus, BarChart3 } from '@lucide/svelte';
    import { Card, Button, Input } from 'positron-components/components/ui';
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';

    interface Props {
        children: import('svelte').Snippet;
    }

    let { children }: Props = $props();
    // Beispiel-Daten für Buckets
    let buckets = [
        {
            id: '1',
            name: 'my-documents',
            description: 'Personal documents and files',
            size: '2.3 GB',
            objectCount: 145,
            created: '2024-01-15',
            region: 'eu-central-1',
            public: false
        },
        {
            id: '2',
            name: 'website-assets',
            description: 'Static assets for website',
            size: '856 MB',
            objectCount: 67,
            created: '2024-02-08',
            region: 'us-east-1',
            public: true
        },
        {
            id: '3',
            name: 'backup-data',
            description: 'System backups and archives',
            size: '15.7 GB',
            objectCount: 892,
            created: '2023-12-03',
            region: 'eu-west-2',
            public: false
        },
        {
            id: '4',
            name: 'media-files',
            description: 'Images, videos and audio files',
            size: '8.2 GB',
            objectCount: 234,
            created: '2024-03-12',
            region: 'ap-southeast-1',
            public: false
        },
        {
            id: '5',
            name: 'logs-storage',
            description: 'Application logs and metrics',
            size: '1.1 GB',
            objectCount: 1823,
            created: '2024-01-28',
            region: 'eu-central-1',
            public: false
        }
    ];

    let searchTerm = $state('');
    let filteredBuckets = $state(buckets);
    
    // Aktive Bucket-ID aus der URL
    let selectedBucketId = $derived($page.params.id);
    let selectedBucket = $derived(buckets.find(b => b.id === selectedBucketId));

    // Suchfunktion als Effect
    $effect(() => {
        if (searchTerm.trim() === '') {
            filteredBuckets = buckets;
        } else {
            filteredBuckets = buckets.filter(bucket =>
                bucket.name.toLowerCase().includes(searchTerm.toLowerCase())
            );
        }
    });

    // Handler für Bucket-Klick
    function handleBucketClick(bucket: any) {
        goto(`/buckets/${bucket.id}`);
    }

    // Handler für neues Bucket
    function handleNewBucket() {
        console.log('Create new bucket');
        // Hier würdest du zu einer Create-Seite navigieren
        // goto('/buckets/new');
    }

    onMount(() => {
        console.log('Buckets layout loaded');
    });
</script>

<div class="h-screen flex flex-col">
    <div class="flex-1 p-6 flex gap-6 min-h-0">
        <!-- Sidebar Card -->
        <Card.Root class="w-80 flex flex-col">
            <Card.Header class="pb-4">
                <Card.Title class="text-lg font-semibold flex items-center gap-2">
                    <Folder class="h-5 w-5 text-primary" />
                    S3 Buckets
                </Card.Title>
            </Card.Header>

            <Card.Content class="flex-1 flex flex-col space-y-4 px-6 pb-6 min-h-0">
                <!-- Suchleiste -->
                <div class="relative">
                    <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground h-4 w-4" />
                    <Input
                        type="text"
                        placeholder="Buckets durchsuchen..."
                        bind:value={searchTerm}
                        class="pl-10"
                    />
                </div>

                <!-- Dashboard Button -->
                <Button onclick={() => goto('/buckets/dashboard')} variant="outline" class="w-full">
                    <BarChart3 class="h-4 w-4 mr-2" />
                    Dashboard
                </Button>

                <!-- Neuer Bucket Button -->
                <Button onclick={handleNewBucket} class="w-full">
                    <Plus class="h-4 w-4 mr-2" />
                    Neues Bucket
                </Button>

                <!-- Bucket-Liste -->
                <div class="flex-1 overflow-y-auto -mx-2 min-h-0">
                    {#if filteredBuckets.length === 0}
                        <div class="p-4 text-center">
                            <Folder class="h-8 w-8 text-muted-foreground mx-auto mb-2" />
                            <p class="text-sm text-muted-foreground">Keine Buckets gefunden</p>
                        </div>
                    {:else}
                        <div class="space-y-1 px-2">
                            {#each filteredBuckets as bucket (bucket.id)}
                                <Button
                                    variant={selectedBucketId === bucket.id ? 'default' : 'ghost'}
                                    class="w-full justify-start p-3 h-auto"
                                    onclick={() => handleBucketClick(bucket)}
                                >
                                    <div class="flex items-center gap-3 w-full">
                                        <Folder class="h-4 w-4 flex-shrink-0" />
                                        <div class="flex flex-col items-start flex-1 min-w-0">
                                            <span class="text-sm font-medium truncate w-full text-left">
                                                {bucket.name}
                                            </span>
                                            <span class="text-xs text-muted-foreground truncate w-full text-left">
                                                {bucket.objectCount} Objekte
                                            </span>
                                        </div>
                                    </div>
                                </Button>
                            {/each}
                        </div>
                    {/if}
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Main Content Card -->
        <Card.Root class="flex-1 min-h-0">
            <Card.Content class="h-full p-0">
                {@render children()}
            </Card.Content>
        </Card.Root>
    </div>
</div>

<style>
    /* Custom scrollbar styling */
    :global(.overflow-y-auto::-webkit-scrollbar) {
        width: 6px;
    }

    :global(.overflow-y-auto::-webkit-scrollbar-track) {
        background: hsl(var(--muted));
        border-radius: 3px;
    }

    :global(.overflow-y-auto::-webkit-scrollbar-thumb) {
        background: hsl(var(--muted-foreground) / 0.3);
        border-radius: 3px;
    }

    :global(.overflow-y-auto::-webkit-scrollbar-thumb:hover) {
        background: hsl(var(--muted-foreground) / 0.5);
    }
</style>