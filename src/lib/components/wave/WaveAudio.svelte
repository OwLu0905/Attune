<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import * as Card from "$lib/components/ui/card/index.js";
  import { onMount } from "svelte";
  import WaveSurfer from "wavesurfer.js";
  import { Play, PlayIcon, TimerIcon, Trash } from "@lucide/svelte";
  import RegionsPlugin, { type Region } from "wavesurfer.js/plugins/regions";
  import { fade, fly, crossfade } from "svelte/transition";
  import { flip } from "svelte/animate";

  let container: HTMLElement;
  let wavesurfer: WaveSurfer;
  // let regions: Region;

  let tmp = $state([
    { id: 2 },
    { id: 5 },
    { id: 4 },
    { id: 7 },
    { id: 8 },
    { id: 66666 },
  ]);
  const [send, receive] = crossfade({
    duration: (d) => Math.sqrt(d * 200),
  });

  onMount(() => {
    const computedStyle = getComputedStyle(document.documentElement);
    const primaryHSL = computedStyle.getPropertyValue("--primary").trim();
    const secondaryHSL = computedStyle.getPropertyValue("--secondary").trim();

    const regions = RegionsPlugin.create();
    wavesurfer = WaveSurfer.create({
      container,
      progressColor: `hsl(${primaryHSL})`,

      waveColor: `hsl(${secondaryHSL})`,
      url: "/arkveld.mp3",
      backend: "WebAudio",
      plugins: [regions],
    });

    wavesurfer.on("decode", () => {
      regions.enableDragSelection({
        drag: true,
      });
    });

    regions.on("region-clicked", (region, e) => {
      e.stopPropagation();
      region.play(true);
    });
  });

  export function trigger() {
    // const a = Math.round(Math.random() * 100);
    // console.log(a);
    // tmp.push(a);
    tmp = [];
  }

  const sortArray = $derived.by(() => {
    return tmp.toSorted((a, b) => {
      return -a + b;
    });
  });
</script>

<Card.Root>
  <Card.Header class="flex flex-row justify-between">
    <div class="flex flex-col gap-1.5">
      <Card.Title>Card Title</Card.Title>
      <Card.Description>Card Description</Card.Description>
    </div>
    <div>
      <Button
        type="button"
        variant="ghost"
        onclick={() => {
          wavesurfer?.play();
        }}
      >
        <Play />
      </Button>
    </div>
  </Card.Header>
  <Card.Content>
    <div bind:this={container}></div>
  </Card.Content>
  <Card.Footer>
    <div class="tabular-nums flex flex-wrap gap-2.5">
      {#each [1, 2, 4, 2, 5, 5, 7, 8, 2, 1, 5, 6] as items, i}
        <div
          class="hover:bg-primary/10 hover:border-primary/20 border-foreground/20 text-foreground cursor-pointer rounded-xl border px-4 py-1 flex items-center gap-1"
        >
          <TimerIcon class="w-[0.75rem]" />
          <div class="text-xs">10:0{items} - 0:{items}3</div>
        </div>
      {/each}
    </div>
  </Card.Footer>
</Card.Root>

<div class="p-6">
  <div class="tabular-nums flex flex-wrap gap-2.5">
    {#each tmp as items, i (items.id)}
      <div
        in:fade
        animate:flip={{ duration: 400 }}
        class="text-xs hover:bg-primary/10 hover:border-primary/20 border-foreground/20 text-foreground cursor-pointer rounded-xl border px-4 py-2.5 gap-1"
      >
        <div class="flex items-center gap-2">
          {i}
          <TimerIcon class="w-[0.75rem]" />
          <div class="text-xs min-w-32">10:0{items.id} - 0:{items.id}3</div>
          <div class="flex gap-6">
            <Trash
              class="w-[0.75rem] text-destructive"
              onclick={() => {
                tmp = tmp.filter((i) => i.id !== items.id);
              }}
            />
            <PlayIcon class="w-[0.75rem] text-primary" />
          </div>
        </div>
        <div class="flex text-xs text-foreground/60">
          <p class="ml-auto leading-6 truncate">{i} recordings</p>
        </div>
      </div>
    {/each}
  </div>
</div>

<Button
  onclick={() => {
    tmp.unshift({ id: Math.round(Math.random() * 1000) });
  }}>123</Button
>
