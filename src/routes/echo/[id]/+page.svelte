<script lang="ts">
import Button from "$lib/components/ui/button/button.svelte";
import * as Card from "$lib/components/ui/card/index.js";
import { onMount } from "svelte";
import WaveSurfer from "wavesurfer.js";
import { Play } from "@lucide/svelte";

let container: HTMLElement;
let wavesurfer: WaveSurfer;

onMount(() => {
	const computedStyle = getComputedStyle(document.documentElement);
	const primaryHSL = computedStyle.getPropertyValue("--primary").trim();
	const secondaryHSL = computedStyle.getPropertyValue("--secondary").trim();

	wavesurfer = WaveSurfer.create({
		container,
		progressColor: `hsl(${primaryHSL})`,
		waveColor: `hsl(${secondaryHSL})`,

		url: "/arkveld.mp3",
		backend: "WebAudio",
	});

	wavesurfer.on("click", () => {
		wavesurfer.play();
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
    <p>Card Footer</p>
  </Card.Footer>
</Card.Root>
