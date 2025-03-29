<script lang="ts">
    import { PauseIcon, PlayIcon, TimerIcon, Trash } from "@lucide/svelte";

    import { flip } from "svelte/animate";
    import { fade } from "svelte/transition";
    import { format, addMilliseconds } from "date-fns";
    import type { Region } from "wavesurfer.js/dist/plugins/regions.js";

    interface Props {
        data: Region[];
        activeRegion: Region | null;
        isPlaying: boolean;
        onPlay: (item: Region) => void;
        onPause: (item: Region) => void;
        onDelete: (item: Region) => void;
    }

    let { data, activeRegion, isPlaying, onPlay, onPause, onDelete }: Props =
        $props();

    function formatDuration(seconds: number) {
        const milliseconds = seconds * 1000;
        const baseDate = new Date(0);
        const date = addMilliseconds(baseDate, milliseconds);
        return format(date, "ss:SS");
    }
</script>

{#each data as item, i (item.id)}
    <div
        in:fade
        animate:flip={{ duration: 400 }}
        class="cursor-pointer gap-1 rounded-xl border border-foreground/20 px-4 py-2.5 text-xs text-foreground transition-all duration-150 ease-linear hover:border-primary/20 hover:bg-primary/10"
    >
        <div class="flex items-center gap-2">
            <TimerIcon class="w-[0.75rem]" />
            <time class="min-w-32 text-xs">
                {formatDuration(item.start)} - {formatDuration(item.end)}
                <span class="text-secondary-foreground/40">(s)</span>
            </time>
            <div class="flex items-center gap-2">
                <Trash
                    class="h-6 w-6 rounded-full px-1.5 text-destructive hover:bg-secondary"
                    onclick={() => {
                        onDelete(item);
                    }}
                />

                {#if activeRegion?.id === item.id && isPlaying}
                    <PauseIcon
                        class="h-6 w-6 select-none rounded-full px-1.5 text-primary hover:bg-secondary"
                        onclick={(e) => {
                            e.stopPropagation();
                            onPause(item);
                        }}
                    />
                {:else}
                    <PlayIcon
                        class="h-6 w-6 select-none rounded-full px-1.5 text-primary hover:bg-secondary"
                        onclick={(e) => {
                            e.stopPropagation();
                            onPlay(item);
                        }}
                    />
                {/if}
            </div>
        </div>
        <div class="flex text-xs text-foreground/60">
            <p class="ml-auto truncate leading-6">{i} recordings</p>
        </div>
    </div>
{/each}
