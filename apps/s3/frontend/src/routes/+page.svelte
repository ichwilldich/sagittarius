<script lang="ts">
  import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
  import { HardDrive, FileText, Server } from "@lucide/svelte/icons";
  import { Chart } from "positron-components/components/ui";
  import { Card } from "positron-components/components/ui";
  import { PieChart, Text } from "layerchart";

  // Speicherverbrauch Daten
  const storageData = [
    { type: "used", size: 127.3, color: "var(--color-used)" },
    { type: "available", size: 372.7, color: "var(--color-available)" }
  ];

  const storageConfig = {
    size: { label: "Storage (GB)" },
    used: { label: "Belegt", color: "var(--chart-1)" },
    available: { label: "Verfügbar", color: "var(--chart-2)" }
  } satisfies Chart.ChartConfig;

  const totalStorage = storageData.reduce((acc, curr) => acc + curr.size, 0);
  const usedPercentage = ((storageData[0].size / totalStorage) * 100).toFixed(1);

  // Dateitypen Daten
  const fileTypeData = [
    { type: "images", count: 1243, color: "var(--color-images)" },
    { type: "documents", count: 856, color: "var(--color-documents)" },
    { type: "videos", count: 432, color: "var(--color-videos)" },
    { type: "archives", count: 289, color: "var(--color-archives)" },
    { type: "other", count: 130, color: "var(--color-other)" }
  ];

  const fileTypeConfig = {
    count: { label: "Anzahl Dateien" },
    images: { label: "Bilder", color: "var(--chart-1)" },
    documents: { label: "Dokumente", color: "var(--chart-2)" },
    videos: { label: "Videos", color: "var(--chart-3)" },
    archives: { label: "Archive", color: "var(--chart-4)" },
    other: { label: "Andere", color: "var(--chart-5)" }
  } satisfies Chart.ChartConfig;

  const totalFiles = fileTypeData.reduce((acc, curr) => acc + curr.count, 0);
</script>

<div class="space-y-6">
  <!-- Header -->
  <div class="flex flex-col gap-2">
    <h1 class="text-3xl font-bold tracking-tight">Dashboard</h1>
    <p class="text-muted-foreground">
      Übersicht über Speicherverbrauch und Dateitypen in Ihren S3 Buckets
    </p>
  </div>

  <!-- KPI Cards -->
  <div class="grid gap-4 md:grid-cols-3">
    <Card.Root>
      <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
        <Card.Title class="text-sm font-medium">Gesamt Speicher</Card.Title>
        <HardDrive class="h-4 w-4 text-muted-foreground" />
      </Card.Header>
      <Card.Content>
        <div class="text-2xl font-bold">{totalStorage.toFixed(1)} GB</div>
        <p class="text-xs text-muted-foreground">
          {usedPercentage}% belegt
        </p>
      </Card.Content>
    </Card.Root>

    <Card.Root>
      <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
        <Card.Title class="text-sm font-medium">Gesamt Dateien</Card.Title>
        <FileText class="h-4 w-4 text-muted-foreground" />
      </Card.Header>
      <Card.Content>
        <div class="text-2xl font-bold">{totalFiles.toLocaleString()}</div>
        <p class="text-xs text-muted-foreground">
          Über alle Buckets
        </p>
      </Card.Content>
    </Card.Root>

    <Card.Root>
      <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
        <Card.Title class="text-sm font-medium">Verfügbarer Speicher</Card.Title>
        <Server class="h-4 w-4 text-muted-foreground" />
      </Card.Header>
      <Card.Content>
        <div class="text-2xl font-bold">{storageData[1].size.toFixed(1)} GB</div>
        <p class="text-xs text-muted-foreground">
          {(100 - parseFloat(usedPercentage)).toFixed(1)}% frei
        </p>
      </Card.Content>
    </Card.Root>
  </div>

  <!-- Charts Grid -->
  <div class="grid gap-6 md:grid-cols-2">
    <!-- Speicherverbrauch Chart -->
    <Card.Root class="flex flex-col">
      <Card.Header class="items-center">
        <Card.Title>Speicherverbrauch</Card.Title>
        <Card.Description>Belegter vs. verfügbarer Speicherplatz</Card.Description>
      </Card.Header>
      <Card.Content class="flex-1">
        <Chart.Container config={storageConfig} class="mx-auto aspect-square max-h-[300px]">
          <PieChart
            data={storageData}
            key="type"
            value="size"
            c="color"
            innerRadius={60}
            padding={28}
            props={{ pie: { motion: "tween" } }}
          >
            {#snippet aboveMarks()}
              <Text
                value={`${storageData[0].size.toFixed(1)} GB`}
                textAnchor="middle"
                verticalAnchor="middle"
                class="fill-foreground text-2xl! font-bold"
                dy={-5}
              />
              <Text
                value="Belegt"
                textAnchor="middle"
                verticalAnchor="middle"
                class="fill-muted-foreground! text-muted-foreground text-sm!"
                dy={15}
              />
              <Text
                value={`${usedPercentage}%`}
                textAnchor="middle"
                verticalAnchor="middle"
                class="fill-muted-foreground! text-muted-foreground text-xs!"
                dy={30}
              />
            {/snippet}
            {#snippet tooltip()}
              <Chart.Tooltip hideLabel />
            {/snippet}
          </PieChart>
        </Chart.Container>
      </Card.Content>
      <Card.Footer class="flex-col gap-2 text-sm">
        <div class="flex items-center gap-2 font-medium leading-none">
          Speicherverbrauch im normalen Bereich <HardDrive class="size-4" />
        </div>
        <div class="text-muted-foreground leading-none">
          {storageData[1].size.toFixed(1)} GB von {totalStorage.toFixed(1)} GB verfügbar
        </div>
      </Card.Footer>
    </Card.Root>

    <!-- Dateitypen Chart -->
    <Card.Root class="flex flex-col">
      <Card.Header class="items-center">
        <Card.Title>Dateitypen Verteilung</Card.Title>
        <Card.Description>Aufschlüsselung nach Dateitypen</Card.Description>
      </Card.Header>
      <Card.Content class="flex-1">
        <Chart.Container config={fileTypeConfig} class="mx-auto aspect-square max-h-[300px]">
          <PieChart
            data={fileTypeData}
            key="type"
            value="count"
            c="color"
            innerRadius={60}
            padding={28}
            props={{ pie: { motion: "tween" } }}
          >
            {#snippet aboveMarks()}
              <Text
                value={String(totalFiles.toLocaleString())}
                textAnchor="middle"
                verticalAnchor="middle"
                class="fill-foreground text-2xl! font-bold"
                dy={-5}
              />
              <Text
                value="Dateien"
                textAnchor="middle"
                verticalAnchor="middle"
                class="fill-muted-foreground! text-muted-foreground text-sm!"
                dy={15}
              />
              <Text
                value="Gesamt"
                textAnchor="middle"
                verticalAnchor="middle"
                class="fill-muted-foreground! text-muted-foreground text-xs!"
                dy={30}
              />
            {/snippet}
            {#snippet tooltip()}
              <Chart.Tooltip hideLabel />
            {/snippet}
          </PieChart>
        </Chart.Container>
      </Card.Content>
      <Card.Footer class="flex-col gap-2 text-sm">
        <div class="flex items-center gap-2 font-medium leading-none">
          Bilder dominieren mit {((fileTypeData[0].count / totalFiles) * 100).toFixed(1)}% <TrendingUpIcon class="size-4" />
        </div>
        <div class="text-muted-foreground leading-none">
          {fileTypeData[0].count.toLocaleString()} Bilder von {totalFiles.toLocaleString()} Dateien
        </div>
      </Card.Footer>
    </Card.Root>
  </div>
</div>

<style>
  :root {
    --color-used: var(--chart-1);
    --color-available: var(--chart-2);
    --color-images: var(--chart-1);
    --color-documents: var(--chart-2);
    --color-videos: var(--chart-3);
    --color-archives: var(--chart-4);
    --color-other: var(--chart-5);
  }
</style>