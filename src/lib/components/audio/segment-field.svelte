<script lang="ts">
    import { cn } from "@/utils";
    import { fade } from "svelte/transition";
    import { Eye, EyeOff, Heart } from "@lucide/svelte";
    import type { SubtitleSegment } from "./types";
    import type { AudioPlayer } from "./audio-player.svelte";
    import type { BookmarkDictationView } from "$lib/tauri";
    import BookOpenIcon from "./book-open-icon.svelte";

    interface Props {
        audioPlayer: AudioPlayer;
        segment: SubtitleSegment;
        hidden: boolean;
        onPause: () => Promise<void>;
        onPlaySection: (
            start: number,
            end: number,
            setEnd?: boolean,
        ) => Promise<void>;
        index: number;
        dictationId: number;
        getDictation: (i: number, st: number) => void;
        combinedList: BookmarkDictationView[];
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
        dictationId,
        getDictation,
        combinedList,
        createBookmarkItem,
        deleteBookmarkItem,
    }: Props = $props();

    let currentTime = $derived(audioPlayer.currentTime);

    let hidden = $derived(isHidenAll);

    let isSelected = $derived(dictationId === index);

    let isCurrentLine = $derived(
        segment.end >= currentTime && currentTime >= segment.start,
    );

    const isDictation = $derived.by(
        () =>
            combinedList.findIndex((i) => i.dictationPosition === +index) > -1,
    );

    const isBookMark = $derived.by(
        () => combinedList.findIndex((i) => i.bookmarkPosition === +index) > -1,
    );
</script>

<div
    class={cn(
        "rounded-md px-2 py-2",
        "group relative flex  items-center",
        "before:top-0 before:-translate-x-2 before:-translate-y-4",
        !hidden && "hover:underline",
        isCurrentLine &&
            "before:text-primary before:transform-transform ease-in before:absolute before:translate-y-1.5 before:duration-300 before:content-['>']",

        isSelected && "bg-gradient-to-br from-red-100 via-pink-50 to-rose-50",
    )}
>
    <div class={cn("flex shrink items-start gap-2")}>
        <div class="mx-1.5 flex items-center gap-2">
            <button
                onclick={() => {
                    getDictation(index, segment.words[0].end);
                }}
            >
                <BookOpenIcon {isDictation} isActive={isSelected} />
            </button>
            {#if isBookMark}
                <div in:fade class="h-6 w-5">
                    <button
                        onclick={() => {
                            deleteBookmarkItem(index);
                        }}
                    >
                        <Heart class="h-6 w-5 stroke-rose-400" />
                    </button>
                </div>
            {:else}
                <div in:fade class="h-6 w-5">
                    <button
                        onclick={() => {
                            createBookmarkItem(index);
                        }}
                    >
                        <Heart class="stroke-primary/30 h-6 w-5 stroke-1" />
                    </button>
                </div>
            {/if}
        </div>
        <div class="flex grow flex-wrap gap-x-1 gap-y-0.5">
            {#each segment?.words as seg}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <span
                    in:fade
                    onclick={(e) => {
                        e.stopPropagation();
                        onPlaySection(seg.start, segment.end, true);
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
