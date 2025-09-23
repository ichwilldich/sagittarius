<script lang="ts">
    import { Folder, Database, Plus } from '@lucide/svelte';
    import { Card, Button } from 'positron-components/components/ui';
    import { goto } from '$app/navigation';

    function handleNewBucket() {
        console.log('Create new bucket');
        // Hier würde eine Modal oder Navigation zu einer Create-Seite stattfinden
    }

    function handleViewAllBuckets() {
        console.log('View all buckets');
    }
</script>

<div class="h-full flex items-center justify-center">
    <div class="text-center space-y-6 p-8 max-w-md">
        <div class="p-6 bg-muted rounded-full w-fit mx-auto">
            <Database class="h-16 w-16 text-muted-foreground" />
        </div>
        <div class="space-y-3">
            <h2 class="text-2xl font-bold">S3 Object Storage</h2>
            <p class="text-muted-foreground">
                Verwalten Sie Ihre Buckets und Dateien in der Cloud. Wählen Sie ein Bucket aus der Seitenleiste aus oder erstellen Sie ein neues.
            </p>
        </div>
        <div class="space-y-3">
            <Button onclick={handleNewBucket} class="w-full">
                <Plus class="h-4 w-4 mr-2" />
                Neues Bucket erstellen
            </Button>
            <Button onclick={handleViewAllBuckets} variant="outline" class="w-full">
                <Folder class="h-4 w-4 mr-2" />
                Alle Buckets anzeigen
            </Button>
        </div>
        
        <Card.Root class="text-left mt-6">
            <Card.Content class="p-4">
                <div class="space-y-2">
                    <h4 class="font-semibold text-sm">Schnellstart</h4>
                    <ul class="text-xs text-muted-foreground space-y-1">
                        <li>• Erstellen Sie Ihr erstes Bucket</li>
                        <li>• Laden Sie Dateien hoch</li>
                        <li>• Verwalten Sie Berechtigungen</li>
                        <li>• Organisieren Sie Ihre Daten</li>
                    </ul>
                </div>
            </Card.Content>
        </Card.Root>
    </div>
</div>