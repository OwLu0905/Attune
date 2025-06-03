<script lang="ts">
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";

    import Button from "$lib/components/ui/button/button.svelte";
    import Slider from "@/components/ui/slider/slider.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Tabs from "@/components/ui/tabs";
    import {
        LoaderCircle,
        Pause,
        Play,
        RotateCcw,
        Volume2,
    } from "@lucide/svelte";
    import ScrollArea from "@/components/ui/scroll-area/scroll-area.svelte";

    import { AudioPlayer } from "./audio-player.svelte";
    import { getSubtitleFile } from "@/utils";
    import ClozeCard from "./cloze-card.svelte";
    import SegmentField from "./segment-field.svelte";

    import type { SubtitleSegment } from "./types";
    import type { AudioItem } from "@/types/audio";

    interface Props {
        audioPath: BlobPart;
        audioItem: AudioItem;
    }

    let { audioPath, audioItem = $bindable() }: Props = $props();

    let audioPlayer: AudioPlayer | undefined = $state(undefined);
    let container: HTMLElement | undefined = $state(undefined);
    let volume = $state(10);

    let subtitles: SubtitleSegment[] = $state([]);

    let visible = $state(true);

    let questionId = $state("0");

    let tabValue = $state<"list" | "swiper">("list");

    onMount(() => {
        async function load() {
            if (!container) return;
            audioPlayer = new AudioPlayer(container);
            await audioPlayer.initialize(audioPath);
            volume = audioPlayer.getVolume() * 100;

            subtitles = await getSubtitleFile(audioItem.id);
        }
        load();

        return () => {
            if (audioPlayer) {
                audioPlayer.destory();
            }
        };
    });

    async function onPlaySection(start: number, end: number) {
        if (!audioPlayer) return;
        audioPlayer.onPlaySection(start, end);
    }
    async function onPause() {
        if (!audioPlayer) return;
        audioPlayer.onPause();
    }
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

<div class="p-6">
    {#if audioPlayer && audioPlayer.isReady}
        <Tabs.Root bind:value={tabValue} class="" activationMode="manual">
            <Tabs.List>
                <Tabs.Trigger
                    value="list"
                    onclick={() => {
                        visible = true;
                    }}>List</Tabs.Trigger
                >
                <Tabs.Trigger
                    value="swiper"
                    onclick={() => {
                        visible = false;
                    }}>Swiper</Tabs.Trigger
                >
            </Tabs.List>
            <Tabs.Content value="list">
                <div transition:fade>
                    <ScrollArea
                        class="text-md flex max-h-80 w-full max-w-full flex-col gap-0.5 overflow-auto bg-stone-100 px-4 py-2 tabular-nums"
                    >
                        <div class="flex flex-col gap-0">
                            {#each subtitles as i, index (index)}
                                <div class="flex w-full gap-4 p-2 shadow-sm">
                                    <span
                                        class="w-4 shrink-0 text-right text-sm leading-6"
                                        >{index + 1}</span
                                    >
                                    <div class="shrink grow">
                                        <SegmentField
                                            {audioPlayer}
                                            segment={i}
                                            hidden={false}
                                        />
                                    </div>
                                    <div class="flex shrink-0 gap-4">
                                        {#if audioPlayer?.isPlaying && i.start <= audioPlayer?.currentTime && audioPlayer?.currentTime <= i.end}
                                            <Pause
                                                class="w-5 text-primary"
                                                onclick={() => {
                                                    onPause();
                                                }}
                                            />
                                        {:else}
                                            <Play
                                                class="w-5 stroke-primary text-primary"
                                                onclick={() => {
                                                    if (!audioPlayer) return;

                                                    const startTime =
                                                        i.start <=
                                                            audioPlayer?.currentTime &&
                                                        audioPlayer?.currentTime <=
                                                            i.end
                                                            ? audioPlayer?.currentTime
                                                            : i.start;

                                                    onPlaySection(
                                                        startTime,
                                                        i.end,
                                                    );
                                                }}
                                            />
                                        {/if}

                                        <RotateCcw
                                            class="w-5 text-lime-500"
                                            onclick={() => {
                                                onPlaySection(i.start, i.end);
                                            }}
                                        />
                                    </div>
                                </div>
                            {/each}
                        </div>
                    </ScrollArea>
                </div>
            </Tabs.Content>

            <Tabs.Content value="swiper">
                {#if tabValue === "swiper"}
                    <ClozeCard
                        {questionId}
                        {audioPlayer}
                        {onPause}
                        {onPlaySection}
                        {subtitles}
                    />
                {/if}
            </Tabs.Content>
        </Tabs.Root>
    {:else}
        <LoaderCircle class="mx-auto animate-spin text-primary" />
    {/if}
</div>
