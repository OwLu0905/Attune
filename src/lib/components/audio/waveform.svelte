<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import * as Card from "$lib/components/ui/card/index.js";
  import { onMount } from "svelte";

  import WaveSurfer from "wavesurfer.js";
  import RegionsPlugin from "wavesurfer.js/dist/plugins/regions.esm.js";
  import type { Region } from "wavesurfer.js/dist/plugins/regions.esm.js";

  import { Play, Scissors } from "@lucide/svelte";

  let container: HTMLElement;
  let ws: WaveSurfer;
  let activeRegion: Region | null = null;
  let regions: RegionsPlugin;

  const random = (min: number, max: number) =>
    Math.random() * (max - min) + min;
  const randomColor = () =>
    `rgba(${random(0, 255)}, ${random(0, 255)}, ${random(0, 255)}, 0.5)`;

  onMount(() => {
    const computedStyle = getComputedStyle(document.documentElement);
    const primaryHSL = computedStyle.getPropertyValue("--primary").trim();
    const secondaryHSL = computedStyle.getPropertyValue("--secondary").trim();

    regions = RegionsPlugin.create();

    ws = WaveSurfer.create({
      container,
      progressColor: `hsl(${primaryHSL})`,
      waveColor: `hsl(${secondaryHSL})`,
      barWidth: 2,
      barGap: 2,
      url: "/arkveld.mp3",
      backend: "WebAudio",
      plugins: [regions],
    });

    regions.enableDragSelection({
      color: randomColor(),
      drag: false,
      id: crypto.randomUUID(),
    });

    regions.on("region-created", (region) => {
      console.log(region);
    });

    regions.on("region-out", (region) => {
      console.log("region-out", region);
      if (activeRegion === region) {
        activeRegion = null;
      }
    });

    regions.on("region-clicked", (region, e) => {
      e.stopPropagation(); // prevent triggering a click on the waveform
      activeRegion = region;
      region.play(true);
      region.setOptions({ color: randomColor() });
    });
    ws.on("interaction", () => {
      activeRegion = null;
    });
  });

  $effect(() => {});
</script>

<Card.Root>
  <Card.Header class="flex flex-row justify-between">
    <div class="flex flex-col gap-1.5">
      <Card.Title>Card Title</Card.Title>
      <Card.Description>Card Description</Card.Description>
    </div>
    <div class="flex gap-2 items-center">
      <Button
        class="text-muted-foreground"
        type="button"
        variant="outline"
        onclick={() => {
          ws?.play();
          // ws.seekTo(0.66);
        }}
      >
        <Play />
      </Button>
      <Button type="button" variant="secondary">
        <Scissors />
        Create Segment
      </Button>
    </div>
  </Card.Header>
  <Card.Content>
    <div bind:this={container}></div>
  </Card.Content>
  <Card.Footer>
    <p>Card Footer</p>
  </Card.Footer>
</Card.Root>
