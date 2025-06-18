<script lang="ts">
    import { fade } from "svelte/transition";

    import ScrollArea from "@/components/ui/scroll-area/scroll-area.svelte";
    import SegmentField from "./segment-field.svelte";

    import { Pause, Play, RotateCcw } from "@lucide/svelte";
    import type { SubtitleSegment } from "./types";
    import type { AudioPlayer } from "./audio-player.svelte";

    interface Props {
        subtitles: SubtitleSegment[];
        audioPlayer: AudioPlayer;
        onPause: () => Promise<void>;
        onPlaySection: (start: number, end: number) => Promise<void>;
    }
    let { subtitles, audioPlayer, onPause, onPlaySection }: Props = $props();
</script>

<div transition:fade>
    <ScrollArea
        class="text-md flex max-h-80 w-full max-w-full flex-col gap-0.5 overflow-auto bg-stone-100 px-4 py-2 tabular-nums"
    >
        <div class="flex flex-col gap-0">
            {#each subtitles as i, index (index)}
                <div class="flex w-full gap-4 p-2">
                    <span class="w-4 shrink-0 text-right text-sm leading-6"
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
                                class="text-primary w-4"
                                onclick={() => {
                                    onPause();
                                }}
                            />
                        {:else}
                            <Play
                                class="stroke-primary text-primary w-4"
                                onclick={() => {
                                    if (!audioPlayer) return;

                                    const startTime =
                                        i.start <= audioPlayer?.currentTime &&
                                        audioPlayer?.currentTime <= i.end
                                            ? audioPlayer?.currentTime
                                            : i.start;

                                    onPlaySection(startTime, i.end);
                                }}
                            />
                        {/if}

                        <RotateCcw
                            class="w-4 text-lime-500"
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
