<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";
    import { getUserContext } from "@/user/userService.svelte";

    import Button from "$lib/components/ui/button/button.svelte";
    import Slider from "@/components/ui/slider/slider.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Tabs from "@/components/ui/tabs";
    import {
        Eye,
        EyeOff,
        LoaderCircle,
        Pause,
        Play,
        RotateCcw,
        Volume2,
        VolumeOff,
    } from "@lucide/svelte";
    import DictationCard from "./dictation-card.svelte";
    import { AudioPlayer } from "./audio-player.svelte";
    import { getSubtitleFile } from "@/utils";

    import type { SubtitleSegment } from "./types";
    import type { AudioItem } from "@/types/audio";
    import ListCard from "./list-card.svelte";
    import { listen } from "@tauri-apps/api/event";

    interface Props {
        audioPath: BlobPart;
        audioItem: AudioItem;
    }

    let { audioPath, audioItem = $bindable() }: Props = $props();

    const { getUser } = getUserContext();
    const user = getUser();

    let audioPlayer: AudioPlayer | undefined = $state(undefined);
    let container: HTMLElement | undefined = $state(undefined);
    let volume = $state(10);
    let subtitles: SubtitleSegment[] = $state([]);
    let visible = $state(true);
    let questionId = $state("0");
    let tabValue = $state<"list" | "swiper">("swiper");
    let isTranscribing = $state(false);
    let hiddenAll = $state(true);

    let prog = $state("");

    onMount(() => {
        async function load() {
            if (!container) return;
            audioPlayer = new AudioPlayer(container);
            await audioPlayer.initialize(audioPath);
            volume = audioPlayer.getVolume() * 100;

            if (audioItem.transcribe === 1) {
                subtitles = await getSubtitleFile(audioItem.id);
            }
        }
        load();

        return () => {
            if (audioPlayer) {
                audioPlayer.destroy();
            }
        };
    });

    async function getSubtitle() {
        try {
            isTranscribing = true;

            // TODO:
            await invoke("start_transcribe_service_streaming", {
                audio_id: audioItem.id,
                model: "small.en",
            });
            audioItem = await invoke("handle_update_audio_transcribe", {
                token: user.accessToken,
                audio_id: audioItem.id,
            });
            subtitles = await getSubtitleFile(audioItem.id);
        } catch (error) {
            console.error(error);
        } finally {
            isTranscribing = false;
        }
    }

    async function onPlaySection(start: number, end: number) {
        if (!audioPlayer) return;
        audioPlayer.onPlaySection(start, end);
    }
    async function onPause() {
        if (!audioPlayer) return;
        audioPlayer.onPause();
    }
    onMount(() => {
        listen("transcription-progress", (event) => {
            const { status, message, progress } = event.payload;
            prog = status + message;
        });
    });
</script>

<Card.Root class="">
    <Card.Header class="flex flex-row justify-between">
        <div class="flex flex-col gap-1.5">
            <Card.Title>{audioItem.title}</Card.Title>
            <Card.Description>{audioItem.id}</Card.Description>
        </div>
    </Card.Header>
    <Card.Content>
        <div bind:this={container}></div>
        <div class="mt-6 flex items-center justify-center gap-2">
            <Button
                disabled={isTranscribing}
                onclick={() => {
                    getSubtitle();
                }}
            >
                {#if isTranscribing}
                    <LoaderCircle class="animate-spin" />
                {/if}
                Transcribe !
            </Button>
            {#if volume === 0}
                <VolumeOff class="text-primary" size={16} />
            {:else}
                <Volume2
                    onclick={() => {
                        volume = 0;
                        audioPlayer?.onMuted();
                    }}
                    class="text-primary"
                    size={16}
                />
            {/if}
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

            <button
                class="text-primary ml-auto h-4 w-4"
                onclick={() => {
                    if (!audioPlayer) return;

                    audioPlayer.onPlayPause();
                }}
            >
                {#if audioPlayer?.isPlaying}
                    <Pause size={16} />
                {:else}
                    <Play size={16} />
                {/if}
            </button>
        </div>
    </Card.Content>

    <Card.Footer class="flex w-full flex-col items-start gap-2">
        <div class="flex gap-2">
            <div class="flex gap-2">
                <div class="flex items-center gap-1">
                    <div class="bg-primary/60 h-2 w-2 rounded-full"></div>
                    <div class="text-xs">Current Segment</div>
                </div>
            </div>
            <div class="text-xs tabular-nums">
                {audioPlayer?.currentTime?.toFixed(2)} (sec)
            </div>
        </div>
    </Card.Footer>
</Card.Root>
<div class="shrink grow overflow-auto px-2 pt-2">
    {#if audioItem.transcribe === 0 && !isTranscribing}
        <Button
            disabled={isTranscribing}
            onclick={() => {
                getSubtitle();
            }}
        >
            {#if isTranscribing}
                <LoaderCircle class="text-primary mx-auto animate-spin" />
            {/if}
            Transcribe !
        </Button>
    {:else if isTranscribing}
        <LoaderCircle class="animate-spin" />
    {:else if audioPlayer && audioPlayer.isReady}
        <Tabs.Root
            bind:value={tabValue}
            activationMode="manual"
            class="relative h-full"
        >
            <Tabs.List>
                <Tabs.Trigger
                    value="swiper"
                    onclick={() => {
                        visible = false;
                    }}
                >
                    Swiper
                </Tabs.Trigger>
                <Tabs.Trigger
                    value="list"
                    onclick={() => {
                        visible = true;
                    }}
                >
                    List
                </Tabs.Trigger>
            </Tabs.List>
            <Tabs.Content value="list">
                <ListCard {subtitles} {audioPlayer} {onPause} {onPlaySection} />
            </Tabs.Content>

            <Tabs.Content value="swiper">
                {#if tabValue === "swiper"}
                    <DictationCard
                        audioId={audioItem.id}
                        {questionId}
                        {audioPlayer}
                        {onPause}
                        {onPlaySection}
                        {subtitles}
                        {hiddenAll}
                    />
                {/if}
            </Tabs.Content>
            <div class="absolute top-0 right-0 flex justify-between gap-4">
                <Button
                    onclick={() => (hiddenAll = !hiddenAll)}
                    variant="outline"
                >
                    {#if hiddenAll}
                        <Eye class="text-primary" />
                    {:else}
                        <EyeOff class="text-primary" />
                    {/if}
                </Button>
            </div>
        </Tabs.Root>
    {:else}
        <LoaderCircle class="text-primary mx-auto animate-spin" />
    {/if}
</div>
