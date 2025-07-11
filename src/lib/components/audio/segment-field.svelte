<script lang="ts">
    import { cn } from "@/utils";
    import { fade } from "svelte/transition";
    import { Eye, EyeOff, Heart, PencilRuler } from "@lucide/svelte";
    import type { SubtitleSegment } from "./types";
    import type { AudioPlayer } from "./audio-player.svelte";
    import type { TBookmarkItem } from "./list-card.svelte";

    interface Props {
        audioPlayer: AudioPlayer;
        segment: SubtitleSegment;
        hidden: boolean;
        onPause: () => Promise<void>;
        onPlaySection: (start: number, end: number) => Promise<void>;
        index: number;
        getDictation: (i: number) => void;
        bookmarkList: TBookmarkItem[];
        createBookmarkItem: (i: number) => Promise<void>;
        deleteBookmarkItem: (i: number) => Promise<void>;
    }
    let {
        audioPlayer,
        segment,
        hidden: isHidenAll,
        onPause,
        onPlaySection,
        index,
        getDictation,
        bookmarkList,
        createBookmarkItem,
        deleteBookmarkItem,
    }: Props = $props();

    let currentTime = $derived(audioPlayer.currentTime);

    let hidden = $derived(isHidenAll);

    let isCurrentLine = $derived(
        segment.end >= currentTime && currentTime >= segment.start,
    );

    const isLove = $derived.by(
        () => bookmarkList.findIndex((i) => i.bookmarkId === +index) > -1,
    );
</script>

<div
    class={cn(
        "group relative flex cursor-pointer items-center",
        "before:top-0 before:-translate-x-2 before:-translate-y-4",
        !hidden && "hover:underline",
        isCurrentLine &&
            "before:text-primary before:transform-transform ease-in before:absolute before:translate-y-0 before:duration-300 before:content-['>']",
    )}
>
    <div class="flex w-full shrink gap-2">
        <div>
            <PencilRuler
                class="h-6 w-4"
                onclick={() => {
                    getDictation(index);
                }}
            />
        </div>
        {#if isLove}
            <div in:fade class="mr-0.5 ml-2">
                <button
                    onclick={() => {
                        deleteBookmarkItem(index);
                    }}
                >
                    <Heart class="h-6 w-4 fill-rose-400 stroke-rose-400" />
                </button>
            </div>
        {:else}
            <div in:fade class="mr-1 ml-2">
                <button
                    onclick={() => {
                        createBookmarkItem(index);
                    }}
                >
                    <Heart class="h-6 w-4 stroke-gray-300 opacity-50" />
                </button>
            </div>
        {/if}
        <div class="flex grow flex-wrap gap-x-1 gap-y-0.5">
            {#each segment?.words as seg}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <span
                    in:fade
                    onclick={(e) => {
                        e.stopPropagation();
                        onPlaySection(seg.start, segment.end);
                    }}
                    class={cn(
                        "rounded-sm",
                        "text-neutral-700 transition-all duration-150 ease-in",
                        hidden &&
                            "bg-neutral-400 text-neutral-400 opacity-10 selection:text-neutral-400",
                        !hidden &&
                            seg.end >= currentTime &&
                            currentTime >= seg.start &&
                            "text-primary bg-neutral-400/10",

                        hidden &&
                            seg.end >= currentTime &&
                            currentTime >= seg.start &&
                            "text-neutral-400/50 opacity-50 selection:text-neutral-400/50",
                    )}
                >
                    {seg.word}
                </span>
            {/each}
        </div>
        <div
            class="transform-all ml-auto flex shrink-0 gap-2 opacity-0 duration-150 ease-out group-hover:opacity-100"
        >
            {#if hidden}
                <Eye
                    class="h-6 w-4"
                    onclick={(e) => {
                        e.stopPropagation();
                        hidden = false;
                    }}
                />
            {:else}
                <EyeOff
                    class="h-6 w-4"
                    onclick={(e) => {
                        e.stopPropagation();
                        hidden = true;
                    }}
                />
            {/if}
        </div>
    </div>
</div>
