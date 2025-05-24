<script lang="ts">
    import { onMount } from "svelte";

    import type { SubtitleSegment } from "./types";
    import type { AudioItem } from "@/types/audio";
    import type { Attachment } from "svelte/attachments";
    import { cn, getSubtitleFile } from "@/utils";
    import ScrollArea from "../ui/scroll-area/scroll-area.svelte";

    let subtitlesByText: SubtitleSegment[] = $state([]);

    interface Props {
        audioItem: AudioItem;
        currentTime: number;
        isPlaying: boolean;
        onClickText: (e: SubtitleSegment["words"][0]) => void;
    }

    let { audioItem, currentTime, onClickText, isPlaying }: Props = $props();

    let scrollMap = new Set();

    // $effect(() => {
    //     return () => {
    //         if (isPlaying) {
    //             scrollMap = new Set();
    //         }
    //     };
    // });

    onMount(async () => {
        subtitlesByText = await getSubtitleFile(audioItem.id);
        // const data = parseSRT(SRT_DATA);
        //
        // subtitlesByText = groupSubtitlesByText(data);
    });

    const observeVisibility = (isCurrent: boolean): Attachment => {
        return (node) => {
            const observer = new IntersectionObserver(
                (entries, observer) => {
                    const entry = entries[0];
                    const currentNode = scrollMap.has(node);
                    if (entry.isIntersecting && isCurrent && !currentNode) {
                        node.scrollIntoView({
                            behavior: "smooth",
                            block: "center",
                        });
                        scrollMap.add(node);
                        observer.unobserve(node);
                    }
                },
                {
                    threshold: 0.1,
                    rootMargin: "0px",
                },
            );
            observer.observe(node);
            return () => {
                observer.disconnect();
            };
        };
    };
</script>

{#snippet subtitleScrollArea(entry: SubtitleSegment["words"][0])}
    <button
        onclick={() => onClickText(entry)}
        class={cn(
            "tansition-all inline px-0.5 tracking-wide duration-300 ease-in-out hover:underline",
            entry.end > currentTime && currentTime >= entry.start
                ? "rounded-2xl bg-violet-300 ring-1 ring-violet-400"
                : "",
        )}
    >
        {entry.word}
    </button>
{/snippet}

{#snippet subtitleStaticArea(entry: SubtitleSegment["words"][0])}
    <button
        onclick={() => onClickText(entry)}
        class={"tansition-all inline px-0.5 tracking-wide duration-300 ease-in-out hover:underline"}
    >
        {entry.word}
    </button>
{/snippet}

<!-- prettier-ignore -->
{#snippet subtitleArea(words: SubtitleSegment['words'], isCurrent: boolean)}
    {#if isCurrent}
        <div
            class={cn(
                "tansition-all m-1 flex flex-row flex-wrap gap-0.5 rounded-lg p-1 duration-100  ease-in-out",
                isCurrent
                    ? "bg-primary/20"
                    : "",
            )}
						{@attach observeVisibility(isCurrent)}
        >
            {#each words as entry, index (index)}
                {@render subtitleScrollArea(entry)}
            {/each}
        </div>
    {:else}
        <div class={"m-1 flex flex-row flex-wrap gap-0.5 p-1"}>
            {#each words as entry, index (index)}
                {@render subtitleStaticArea(entry)}
            {/each}
        </div>
    {/if}
{/snippet}

<ScrollArea
    class="text-md flex max-h-80 w-full max-w-96 flex-col gap-0.5 overflow-auto bg-stone-100 px-4 py-2 tabular-nums"
>
    {#each subtitlesByText as i, index (i?.text)}
        {@render subtitleArea(
            i.words,
            i.end > currentTime && currentTime >= i.start,
        )}
    {/each}
</ScrollArea>
