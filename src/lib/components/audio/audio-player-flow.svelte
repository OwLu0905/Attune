<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import Slider from "@/components/ui/slider/slider.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import type { AudioItem } from "@/types/audio";
    import {
        LoaderCircle,
        Pause,
        Play,
        Scissors,
        Volume,
        Volume2,
    } from "@lucide/svelte";
    import { onMount } from "svelte";
    import { AudioPlayer } from "./audio-player.svelte";

    interface Props {
        audioPath: BlobPart;
        audioItem: AudioItem;
    }

    let { audioPath, audioItem = $bindable() }: Props = $props();

    let audioPlayer: AudioPlayer | undefined = $state(undefined);
    let container: HTMLElement | undefined = $state(undefined);
    let volume = $state(10);

    onMount(() => {
        async function load() {
            if (!container) return;
            audioPlayer = new AudioPlayer(container);
            await audioPlayer.initialize(audioPath);
            volume = audioPlayer.getVolume() * 100;
        }
        load();

        return () => {
            if (audioPlayer) {
                audioPlayer.destory();
            }
        };
    });
</script>

<Card.Root>
    <Card.Header class="flex flex-row justify-between">
        <div class="flex flex-col gap-1.5">
            <Card.Title>{audioItem.title}</Card.Title>
            <Card.Description>{audioItem.id}</Card.Description>
        </div>
    </Card.Header>
    <Card.Content>
        <div bind:this={container}></div>
        <div class="mt-8 flex items-center justify-center gap-2">
            <Volume2 class="text-primary" />
            <Slider
                type="single"
                max={100}
                step={1}
                min={0}
                bind:value={volume}
                class="mb-0 max-w-[70%]"
                onValueCommit={(e) => {
                    audioPlayer?.onSetVolume(e / 100);
                }}
            />

            <Button
                class="ml-auto text-muted-foreground"
                type="button"
                size="sm"
                variant="outline"
                onclick={() => {
                    if (!audioPlayer) return;

                    audioPlayer.onPlayPause();
                }}
            >
                {#if audioPlayer?.isPlaying}
                    <Pause />
                {:else}
                    <Play />
                {/if}
            </Button>
        </div>
    </Card.Content>

    <Card.Footer class="flex w-full flex-col items-start gap-2">
        <div class="flex gap-4">
            <div class="flex gap-4">
                <div class="flex items-center gap-2">
                    <div class="h-4 w-4 rounded-full bg-violet-200"></div>
                    <div class="text-sm">Current Segment</div>
                </div>
            </div>
            <div class="text-sm tabular-nums">
                {audioPlayer?.currentTime?.toFixed(2)}(sec)
            </div>
        </div>
    </Card.Footer>
</Card.Root>
