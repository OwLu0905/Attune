<script lang="ts">
    import { fade } from "svelte/transition";

    import Button from "$lib/components/ui/button/button.svelte";
    import * as Select from "@/components/ui/select";

    import SegmentField from "./segment-field.svelte";
    import Textarea from "@/components/ui/textarea/textarea.svelte";

    import {
        ChevronLeft,
        ChevronRight,
        Eye,
        EyeOff,
        Pause,
        Play,
        RotateCcw,
    } from "@lucide/svelte";

    import type { AudioPlayer } from "./audio-player.svelte";
    import type { SubtitleSegment } from "./types";

    interface Props {
        questionId: string;
        audioPlayer: AudioPlayer;
        onPause: () => Promise<void>;
        onPlaySection: (start: number, end: number) => Promise<void>;
        subtitles: SubtitleSegment[];
    }
    let { questionId, audioPlayer, onPause, onPlaySection, subtitles }: Props =
        $props();

    let hiddenAll = $state(true);

    let hiddenItem = $derived.by(() => {
        questionId;
        return hiddenAll;
    });

    const items = $derived(
        Array.from({ length: subtitles.length }, (_, i) => `${i}`),
    );

    $effect(() => {
        questionId;
        return () => {
            audioPlayer.onPause();
        };
    });

    $effect(() => {
        if (
            audioPlayer?.currentTime > subtitles?.[+questionId].end &&
            audioPlayer.isPlaying
        ) {
            audioPlayer.onPause();
        }
    });
</script>

<div class="flex flex-col gap-1 px-4 py-2 tabular-nums">
    <div class="flex justify-between gap-4">
        <Button
            variant="outline"
            size="sm"
            onclick={() => {
                if (+questionId === 0) return;
                questionId = `${+questionId - 1}`;
            }}
        >
            <ChevronLeft />
        </Button>
        <div class="flex items-center gap-4 text-sm">
            <Button onclick={() => (hiddenAll = !hiddenAll)} variant="outline">
                {#if hiddenAll}
                    <Eye class="text-primary" />
                {:else}
                    <EyeOff class="text-primary" />
                {/if}
            </Button>
            <Select.Root type="single" bind:value={questionId}>
                <Select.Trigger class="w-20">{+questionId + 1}</Select.Trigger>
                <Select.Content>
                    {#each items as s}
                        <Select.Item value={s}>{+s + 1}</Select.Item>
                    {/each}
                </Select.Content>
            </Select.Root>
        </div>
        <Button
            variant="outline"
            size="sm"
            onclick={() => {
                if (+questionId === subtitles?.length - 1) {
                    return;
                }
                questionId = `${+questionId + 1}`;
            }}
        >
            <ChevronRight />
        </Button>
    </div>

    <div class="shrink grow">
        {#key questionId}
            <section class=" flex flex-col gap-2 bg-card p-4">
                <div class="flex flex-wrap gap-1 p-1 tracking-wide" in:fade>
                    <SegmentField
                        {audioPlayer}
                        segment={subtitles?.[+questionId]}
                        hidden={hiddenItem}
                    />
                </div>
                <div class="w-full">
                    <Textarea />
                </div>

                <div class="flex items-center justify-center gap-2">
                    <Button
                        onclick={() => (hiddenItem = !hiddenItem)}
                        variant="link"
                    >
                        {#if hiddenItem}
                            <Eye class="text-primary" />
                        {:else}
                            <EyeOff class="text-primary" />
                        {/if}
                    </Button>

                    <Button
                        variant="link"
                        onclick={() => {
                            if (!audioPlayer) return;

                            if (audioPlayer.isPlaying) {
                                onPause();
                            } else {
                                const startTime =
                                    subtitles?.[+questionId].start <=
                                        audioPlayer?.currentTime &&
                                    audioPlayer?.currentTime <=
                                        subtitles?.[+questionId].end
                                        ? audioPlayer?.currentTime
                                        : subtitles?.[+questionId].start;

                                onPlaySection(
                                    startTime,
                                    subtitles?.[+questionId].end,
                                );
                            }
                        }}
                    >
                        {#if audioPlayer?.isPlaying}
                            <Pause class="text-primary" />
                        {:else}
                            <Play class="text-primary" />
                        {/if}
                    </Button>
                    <Button variant="link">
                        <RotateCcw
                            class="w-5 text-lime-500"
                            onclick={() => {
                                onPlaySection(
                                    subtitles?.[+questionId].start,
                                    subtitles?.[+questionId].end,
                                );
                            }}
                        />
                    </Button>
                </div>
            </section>
        {/key}
    </div>
</div>
