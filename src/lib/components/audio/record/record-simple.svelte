<script lang="ts">
    import { onDestroy } from "svelte";
    import { cn } from "@/utils";
    import { AudioRecorderSimple } from "./audio-recorder-simple.svelte";
    import RecordAudioPlayer from "./record-audio-player.svelte";
    import RecordWaveAnimate from "./record-wave-animate.svelte";
    import type { SubtitleSegment } from "../types";
    import type { RecordHistoryData } from "./record-history-data.svelte";
    import type { AudioPlayer } from "../audio-player.svelte";
    import NextButton from "@/components/editor/next-button.svelte";
    import Button from "@/components/ui/button/button.svelte";
    import {
        Loader,
        Mic,
        Music,
        Pause,
        Play,
        Save,
        type Icon as IconType,
    } from "@lucide/svelte";
    import { fade, slide } from "svelte/transition";
    import Checkbox from "@/components/ui/checkbox/checkbox.svelte";

    interface Props {
        audioId: string;
        dictationId: number;
        dictationItem: SubtitleSegment;
        recordData: RecordHistoryData;
        audioPlayer: AudioPlayer;
        onPause: () => Promise<void>;
        onPlaySection: (start: number, end: number) => Promise<void>;
        onPrev: () => void;
        onNext: () => void;
    }

    let {
        audioId,
        dictationId,
        dictationItem,
        recordData,
        audioPlayer,
        onPause,
        onPlaySection,
        onPrev,
        onNext,
    }: Props = $props();

    let recorder = new AudioRecorderSimple();

    let step = $state(0);

    let recordState: "echo" | "record" = $state("record");
    let check = $state(false);

    const handleRecord = async (): Promise<void> => {
        try {
            await recorder.onRecord();
        } catch (error) {
            console.error("Recording failed:", error);
            alert("Recording failed. Please check microphone permissions.");
        }
    };

    onDestroy(() => {
        recorder.destroy();
    });
</script>

{#snippet stepIcon(count: number, Icon: typeof IconType)}
    <button
        class={cn(
            "m-0.5 flex h-15 w-15 translate-x-1/2 items-center justify-center gap-1 rounded-xl transition-all duration-300 ease-in-out",
            "hover:bg-white/5",
            count === step && "text-primary/30",
        )}
        style={`transform:translateX(calc(${(count - step) * 100 - count * 100}% - ${count * 2}px))`}
        onclick={() => {
            step = (step + 1) % 4;
        }}
        aria-label="mic"
    >
        <Icon
            class={cn(
                "text-foreground w-32",
                count !== step && "scale-75",
                count === step && "animate-pulse",
            )}
        />
    </button>
{/snippet}

<div class="w-full py-4">
    <div class="flex gap-4 pb-4">
        <div
            class="relative flex max-w-sm shrink-0 flex-col items-center gap-2"
        >
            <div class="absolute top-0 right-0 z-5">
                <Checkbox
                    bind:checked={
                        () => check,
                        (c) => {
                            check = c;
                            if (c) {
                                recordState = "echo";
                            } else {
                                recordState = "record";
                            }
                        }
                    }
                />
            </div>
            <div class="flex flex-col items-center gap-1">
                {#if recordState === "record"}
                    <div class="control-buttons" in:fade>
                        {#if !recorder.isRecording}
                            <button
                                in:slide
                                out:slide
                                class={cn(
                                    "group flex h-16 w-16 items-center justify-center rounded-xl transition-colors",
                                    recorder.isRecording
                                        ? "bg-none"
                                        : "bg-none hover:bg-black/5 dark:hover:bg-white/5",
                                )}
                                onclick={handleRecord}
                                disabled={recorder.isRecording}
                                aria-label="mic"
                            >
                                <svg
                                    class="h-6 w-6 text-black/90 dark:text-white/90"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
                                    />
                                </svg>
                            </button>
                        {:else}
                            <button
                                in:slide
                                out:slide
                                class={cn(
                                    "group flex h-16 w-16 items-center justify-center rounded-xl transition-colors",
                                    recorder.isRecording
                                        ? "bg-none"
                                        : "bg-none hover:bg-black/5 dark:hover:bg-white/5",
                                )}
                                onclick={() => recorder.onFinish()}
                                aria-label="stop"
                            >
                                <div
                                    class="pointer-events-auto h-6 w-6 animate-spin cursor-pointer rounded-sm bg-black dark:bg-white"
                                    style="animation-duration: 3s;"
                                ></div>
                            </button>
                        {/if}
                    </div>
                {:else}
                    <div
                        in:fade
                        class="relative mx-auto flex h-16 w-32 overflow-hidden"
                    >
                        {@render stepIcon(0, Play)}
                        {@render stepIcon(1, Music)}
                        {@render stepIcon(2, Loader)}
                        {@render stepIcon(3, Mic)}
                    </div>
                {/if}

                <span
                    class={cn(
                        "font-mono text-sm transition-opacity duration-300",
                        recorder.isRecording
                            ? "text-foreground/70"
                            : "text-foreground/30",
                    )}
                >
                    {recorder.currentTime}
                </span>
            </div>

            <RecordWaveAnimate isLoading={recorder.isRecording} />

            <!-- <p class="h-4 text-xs text-black/70 dark:text-white/70"> -->
            <!--     {recorder.isRecording ? "Recording..." : "Click to speak"} -->
            <!-- </p> -->

            <div class="flex shrink-0 justify-center gap-2">
                <NextButton variant="prev" onclick={onPrev} />

                <Button
                    size="sm"
                    variant="ghost"
                    onclick={() => {
                        if (audioPlayer?.isPlaying) {
                            onPause();
                        } else {
                            onPlaySection(
                                dictationItem.start,
                                dictationItem.end,
                            );
                        }
                    }}
                >
                    {#if audioPlayer?.isPlaying}
                        <Pause />
                    {:else}
                        <Play />
                    {/if}
                </Button>
                <NextButton variant="next" onclick={onNext} />
            </div>
        </div>
        <div class="flex grow flex-col">
            <div class="grow overflow-hidden px-4 pb-2.5">
                <div class="text-muted-foreground">
                    {dictationItem.text}
                </div>
            </div>

            {#if recorder.recordedUrl}
                <div
                    in:fade
                    class="bg-card border-border my-2 flex items-center gap-1 rounded-md border"
                >
                    <RecordAudioPlayer
                        src={recorder.recordedUrl}
                        title={"ree"}
                        onDelete={() => {
                            recorder.deleteRecord();
                        }}
                    />
                    <Button
                        variant="ghost"
                        onclick={async () => {
                            await recorder.onSaveFile(audioId, dictationId);
                            await recorder.deleteRecord();
                            await recordData.updateData(audioId, dictationId);
                        }}
                    >
                        <Save />
                    </Button>
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }

    .animate-spin {
        animation: spin 1s linear infinite;
    }
</style>
