<script lang="ts">
    import { HardDrive, Calendar, Globe, Lock, Database, Folder } from '@lucide/svelte';
    import { Card, Badge, Button } from 'positron-components/components/ui';

    interface Props {
        selectedBucket?: any;
    }

    let { selectedBucket }: Props = $props();

    // Formatierung für Datum
    function formatDate(dateString: string) {
        return new Date(dateString).toLocaleDateString('de-DE', {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
        });
    }
</script>

{#if selectedBucket}
    <!-- Bucket Details -->
    <div class="p-6 space-y-6">
        <!-- Header Card -->
        <Card.Root>
            <Card.Header>
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div class="p-2 bg-primary/10 rounded-lg">
                            <Folder class="h-6 w-6 text-primary" />
                        </div>
                        <div>
                            <Card.Title class="text-2xl">{selectedBucket.name}</Card.Title>
                            <Card.Description class="mt-1">{selectedBucket.description}</Card.Description>
                        </div>
                    </div>
                    <Badge variant={selectedBucket.public ? 'default' : 'secondary'} class="rounded-full">
                        {#if selectedBucket.public}
                            <Globe class="h-3 w-3 mr-1" />
                            Öffentlich
                        {:else}
                            <Lock class="h-3 w-3 mr-1" />
                            Privat
                        {/if}
                    </Badge>
                </div>
            </Card.Header>
        </Card.Root>

        <!-- Stats Cards -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <Card.Root class="hover:shadow-md transition-shadow">
                <Card.Content class="p-6">
                    <div class="flex items-center gap-3">
                        <div class="p-2 bg-blue-100 rounded-lg">
                            <HardDrive class="h-5 w-5 text-blue-600" />
                        </div>
                        <div>
                            <div class="text-2xl font-bold">{selectedBucket.size}</div>
                            <div class="text-sm text-muted-foreground">Größe</div>
                        </div>
                    </div>
                </Card.Content>
            </Card.Root>

            <Card.Root class="hover:shadow-md transition-shadow">
                <Card.Content class="p-6">
                    <div class="flex items-center gap-3">
                        <div class="p-2 bg-green-100 rounded-lg">
                            <Database class="h-5 w-5 text-green-600" />
                        </div>
                        <div>
                            <div class="text-2xl font-bold">{selectedBucket.objectCount.toLocaleString()}</div>
                            <div class="text-sm text-muted-foreground">Objekte</div>
                        </div>
                    </div>
                </Card.Content>
            </Card.Root>

            <Card.Root class="hover:shadow-md transition-shadow">
                <Card.Content class="p-6">
                    <div class="flex items-center gap-3">
                        <div class="p-2 bg-purple-100 rounded-lg">
                            <Globe class="h-5 w-5 text-purple-600" />
                        </div>
                        <div>
                            <div class="text-lg font-bold">{selectedBucket.region}</div>
                            <div class="text-sm text-muted-foreground">Region</div>
                        </div>
                    </div>
                </Card.Content>
            </Card.Root>

            <Card.Root class="hover:shadow-md transition-shadow">
                <Card.Content class="p-6">
                    <div class="flex items-center gap-3">
                        <div class="p-2 bg-orange-100 rounded-lg">
                            <Calendar class="h-5 w-5 text-orange-600" />
                        </div>
                        <div>
                            <div class="text-sm font-medium">{formatDate(selectedBucket.created)}</div>
                            <div class="text-sm text-muted-foreground">Erstellt</div>
                        </div>
                    </div>
                </Card.Content>
            </Card.Root>
        </div>

        <!-- Additional Info Card -->
        <Card.Root>
            <Card.Header>
                <Card.Title class="text-lg">Bucket-Informationen</Card.Title>
            </Card.Header>
            <Card.Content class="space-y-4">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="space-y-2">
                        <label class="text-sm font-medium text-muted-foreground">Bucket-Name</label>
                        <div class="p-3 bg-muted rounded-lg">
                            <code class="text-sm">{selectedBucket.name}</code>
                        </div>
                    </div>
                    <div class="space-y-2">
                        <label class="text-sm font-medium text-muted-foreground">Region</label>
                        <div class="p-3 bg-muted rounded-lg">
                            <code class="text-sm">{selectedBucket.region}</code>
                        </div>
                    </div>
                </div>
                
                <div class="border-t pt-4">
                    <div class="space-y-2">
                        <label class="text-sm font-medium text-muted-foreground">Beschreibung</label>
                        <p class="text-sm">{selectedBucket.description}</p>
                    </div>
                </div>
            </Card.Content>
        </Card.Root>
    </div>
{:else}
    <!-- Placeholder -->
    <div class="h-full flex items-center justify-center">
        <div class="text-center space-y-4 p-8">
            <div class="p-4 bg-muted rounded-full w-fit mx-auto">
                <Folder class="h-12 w-12 text-muted-foreground" />
            </div>
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">Wählen Sie ein Bucket aus</h3>
                <p class="text-muted-foreground max-w-md">
                    Klicken Sie auf ein Bucket in der Seitenleiste, um detaillierte Informationen und Statistiken anzuzeigen
                </p>
            </div>
            <Button variant="outline">
                <Database class="h-4 w-4 mr-2" />
                Neues Bucket erstellen
            </Button>
        </div>
    </div>
{/if}