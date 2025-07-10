<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy, onMount } from "svelte";
    import { getUserContext } from "@/user/userService.svelte";

    import Button from "$lib/components/ui/button/button.svelte";
    import Slider from "@/components/ui/slider/slider.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as DropdownMenu from "@/components/ui/dropdown-menu";
    import {
        Bot,
        Languages,
        LoaderCircle,
        Pause,
        Play,
        Settings,
        Volume2,
        VolumeOff,
    } from "@lucide/svelte";
    import { AudioPlayer } from "./audio-player.svelte";
    import ListCard from "./list-card.svelte";

    import { getSubtitleFile } from "@/utils";

    import type { SubtitleSegment } from "./types";
    import type { AudioItem } from "@/types/audio";

    interface Props {
        videoPath: BlobPart;
        audioItem: AudioItem;
    }

    let { audioItem = $bindable(), videoPath }: Props = $props();

    const { getUser } = getUserContext();
    const user = getUser();

    let audioPlayer: AudioPlayer | undefined = $state.raw(undefined);
    let container: HTMLElement | undefined = $state.raw(undefined);
    let volume = $state(10);
    let subtitles: SubtitleSegment[] = $state([]);
    let isTranscribing = $state(false);

    let videoRef: HTMLVideoElement;
    let videoUrl = $derived.by(() => {
        const blob = new Blob([videoPath], { type: "video/mp4" });
        return URL.createObjectURL(blob);
    });
    let prog = $state("");

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
        async function load() {
            if (!container) return;
            if (audioPlayer) {
                audioPlayer.destroy();
            }
            audioPlayer = new AudioPlayer(container, videoRef);

            await audioPlayer.initialize(undefined, () => {
                if (!audioPlayer) return;
                volume = audioPlayer.getVolume() * 100;
            });

            if (videoPath && "mediaSession" in navigator) {
                navigator.mediaSession.metadata = new MediaMetadata({
                    title: audioItem.id,
                });

                // Set up media controls to work with WaveSurfer
                navigator.mediaSession.setActionHandler("play", () => {
                    audioPlayer?.onPlay();
                });
            }

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

    onMount(() => {
        listen("transcription-progress", (event) => {
            // TODO: fix types
            const { status, message, progress } = event.payload;
            prog = status + message;
        });
    });

    onDestroy(() => {
        if (videoUrl) {
            URL.revokeObjectURL(videoUrl);
        }
    });
</script>

<Card.Root class="h-full">
    <Card.Header class="flex flex-row justify-between">
        <div class="flex flex-col gap-1.5">
            <Card.Title>{audioItem.title}</Card.Title>
            <Card.Description class="text-xs tabular-nums"
                >{audioItem.lastUsedAt} - {audioItem.id} - {audioItem.url}</Card.Description
            >
        </div>
        <div>
            {#if audioItem.transcribe === 0 && !isTranscribing}
                <Button
                    disabled={isTranscribing}
                    onclick={() => {
                        getSubtitle();
                    }}
                >
                    {#if isTranscribing}
                        <LoaderCircle
                            class="text-primary mx-auto animate-spin"
                        />
                    {:else}
                        <Bot />
                    {/if}
                    Get Transcribe
                </Button>
            {:else}
                <DropdownMenu.Root>
                    <DropdownMenu.Trigger
                        class="transition-transform duration-200 hover:rotate-30 data-[state=open]:rotate-30"
                    >
                        <Settings size={20} />
                    </DropdownMenu.Trigger>
                    <DropdownMenu.Content side="left" align="start">
                        <DropdownMenu.Group>
                            <DropdownMenu.Item
                                onclick={() => {
                                    // TODO: update subtitle => add alert modal
                                    // getSubtitle();
                                }}
                                disabled={isTranscribing}
                            >
                                {#snippet child({ props })}
                                    <span {...props}>
                                        <Languages />
                                        Transcribe
                                    </span>
                                {/snippet}
                            </DropdownMenu.Item>
                        </DropdownMenu.Group>
                    </DropdownMenu.Content>
                </DropdownMenu.Root>
            {/if}
        </div>
    </Card.Header>

    <Card.Content class="shrink grow overflow-hidden">
        <div class="flex h-full gap-6 py-8">
            <div class="w-142 shrink-0">
                <!-- svelte-ignore a11y_media_has_caption -->
                <video
                    bind:this={videoRef}
                    src={videoUrl}
                    controls
                    playsinline
                    class="rounded-xl"
                    style="width: 100%; width: 480px; margin: 4px auto; display: block;"
                >
                </video>
                <div bind:this={container}></div>

                <div class="mt-6 flex items-center justify-center gap-2">
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
            </div>

            <ListCard
                {audioItem}
                {subtitles}
                {audioPlayer}
                {onPause}
                {onPlaySection}
            />
        </div>
    </Card.Content>

    <Card.Footer
        class="flex w-full shrink-0 basis-10 flex-col items-center justify-center gap-2 border-t"
    >
        <div class="flex gap-2">
            <div class="flex items-center gap-1">
                <div class="bg-primary/60 h-2 w-2 rounded-full"></div>
                <div class="text-xs">Current Segment</div>
            </div>
            <div class="text-xs tabular-nums">
                {audioPlayer?.currentTime?.toFixed(2)} (sec)
            </div>
        </div>
    </Card.Footer>
</Card.Root>
