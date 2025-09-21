<script lang="ts">
    import { onMount } from 'svelte';
    import { Search, Folder } from '@lucide/svelte';

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

    let searchTerm = '';
    let filteredBuckets = buckets;
    let selectedBucket = null;

    // Suchfunktion
    $: {
        if (searchTerm.trim() === '') {
            filteredBuckets = buckets;
        } else {
            filteredBuckets = buckets.filter(bucket =>
                bucket.name.toLowerCase().includes(searchTerm.toLowerCase())
            );
        }
    }

    // Handler für Bucket-Klick
    function handleBucketClick(bucket: any) {
        selectedBucket = bucket;
        console.log('Bucket selected:', bucket);
    }

    onMount(() => {
        console.log('Buckets page loaded');
    });
</script>

<div class="h-full flex">
    <!-- Sidebar -->
    <div class="w-80 bg-gray-50 border-r border-gray-200 flex flex-col">
        <!-- Sidebar Header -->
        <div class="p-4 border-b border-gray-200">
            <h2 class="text-lg font-semibold text-gray-900">S3 Buckets</h2>
        </div>

        <!-- Suchleiste -->
        <div class="p-4">
            <div class="relative">
                <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4" />
                <input
                    type="text"
                    placeholder="Buckets durchsuchen..."
                    bind:value={searchTerm}
                    class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
            </div>
        </div>

        <!-- Bucket-Liste -->
        <div class="flex-1 overflow-y-auto">
            {#if filteredBuckets.length === 0}
                <div class="p-4 text-center text-gray-500">
                    <p>Keine Buckets gefunden</p>
                </div>
            {:else}
                <div class="p-2">
                    {#each filteredBuckets as bucket (bucket.id)}
                        <div 
                            class="flex items-center gap-3 p-3 rounded-lg cursor-pointer hover:bg-gray-100 transition-colors duration-150 {selectedBucket?.id === bucket.id ? 'bg-blue-50 border border-blue-200' : ''}"
                            on:click={() => handleBucketClick(bucket)}
                            on:keydown={(e) => e.key === 'Enter' && handleBucketClick(bucket)}
                            role="button"
                            tabindex="0"
                        >
                            <Folder class="h-5 w-5 text-gray-600 flex-shrink-0" />
                            <span class="text-sm font-medium text-gray-900 truncate">
                                {bucket.name}
                            </span>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex flex-col overflow-hidden">
        {#if selectedBucket}
            <!-- Bucket Details -->
            <div class="p-6">
                <div class="mb-6">
                    <h1 class="text-2xl font-bold text-gray-900 flex items-center gap-2">
                        <Folder class="h-6 w-6 text-blue-600" />
                        {selectedBucket.name}
                    </h1>
                    <p class="text-gray-600 mt-1">{selectedBucket.description}</p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
                    <div class="bg-white p-4 rounded-lg border border-gray-200">
                        <div class="text-2xl font-bold text-gray-900">{selectedBucket.size}</div>
                        <div class="text-sm text-gray-600">Größe</div>
                    </div>
                    <div class="bg-white p-4 rounded-lg border border-gray-200">
                        <div class="text-2xl font-bold text-gray-900">{selectedBucket.objectCount}</div>
                        <div class="text-sm text-gray-600">Objekte</div>
                    </div>
                    <div class="bg-white p-4 rounded-lg border border-gray-200">
                        <div class="text-2xl font-bold text-gray-900">{selectedBucket.region}</div>
                        <div class="text-sm text-gray-600">Region</div>
                    </div>
                    <div class="bg-white p-4 rounded-lg border border-gray-200">
                        <div class="text-2xl font-bold text-gray-900">
                            <span class="px-2 py-1 text-xs rounded-full {selectedBucket.public ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'}">
                                {selectedBucket.public ? 'Öffentlich' : 'Privat'}
                            </span>
                        </div>
                        <div class="text-sm text-gray-600">Sichtbarkeit</div>
                    </div>
                </div>
            </div>
        {:else}
            <!-- Placeholder wenn kein Bucket ausgewählt -->
            <div class="flex-1 flex items-center justify-center">
                <div class="text-center">
                    <Folder class="h-16 w-16 text-gray-400 mx-auto mb-4" />
                    <h3 class="text-lg font-medium text-gray-900 mb-2">Wählen Sie ein Bucket aus</h3>
                    <p class="text-gray-600">Klicken Sie auf ein Bucket in der Seitenleiste, um Details anzuzeigen</p>
                </div>
            </div>
        {/if}
    </div>
</div>

<style>
    /* Custom scrollbar styling für Sidebar */
    :global(.overflow-y-auto::-webkit-scrollbar) {
        width: 6px;
    }

    :global(.overflow-y-auto::-webkit-scrollbar-track) {
        background: #f1f5f9;
        border-radius: 3px;
    }

    :global(.overflow-y-auto::-webkit-scrollbar-thumb) {
        background: #cbd5e1;
        border-radius: 3px;
    }

    :global(.overflow-y-auto::-webkit-scrollbar-thumb:hover) {
        background: #94a3b8;
    }
</style>