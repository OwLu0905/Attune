<script lang="ts">
    import { onMount } from "svelte";
    import { SvelteSet } from "svelte/reactivity";
    import Button from "@/components/ui/button/button.svelte";
    import ScrollArea from "@/components/ui/scroll-area/scroll-area.svelte";
    import { Pause, Play } from "@lucide/svelte";
    import { cn, getSubtitleFile } from "@/utils";

    import type { SubtitleSegment } from "./types";
    import type { AudioItem } from "@/types/audio";
    import type { Attachment } from "svelte/attachments";

    let subtitlesByText: SubtitleSegment[] = $state([]);

    interface Props {
        audioItem: AudioItem;
        currentTime: number;
        isPlaying: boolean;
        hidden: boolean;
        onPlaySubtitleSegment: (e: SubtitleSegment, pause: boolean) => void;
        onClickText: (e: SubtitleSegment["words"][0], toPlay?: boolean) => void;
    }

    let {
        audioItem,
        currentTime,
        onClickText,
        onPlaySubtitleSegment,
        isPlaying,
        hidden = true,
    }: Props = $props();

    let scrollMap = new SvelteSet();

    $effect(() => {
        if (!isPlaying) return;

        return () => {
            scrollMap.clear();
        };
    });

    onMount(async () => {
        subtitlesByText = await getSubtitleFile(audioItem.id);
    });

    const observeVisibility = (isCurrent: boolean): Attachment => {
        return (node) => {
            const observer = new IntersectionObserver(
                (entries, observer) => {
                    const entry = entries[0];

                    if (entry.isIntersecting) {
                        scrollMap.add(node);
                    }
                    const currentNode = scrollMap.has(node);
                    if (
                        !entry.isIntersecting &&
                        isCurrent &&
                        !currentNode &&
                        isPlaying
                    ) {
                        node.scrollIntoView({
                            behavior: "instant",
                            block: "center",
                        });
                        observer.unobserve(node);
                    }
                },
                {
                    threshold: 1.0,
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

{#snippet subtitleScrollArea(
    entry: SubtitleSegment["words"][0],
    hidden: boolean,
)}
    <button
        onclick={(e) => {
            e.stopPropagation();
            onClickText(entry);
        }}
        class={cn(
            "tansition-all inline px-0.5 tracking-wide duration-300 ease-in-out hover:underline",
            entry.end > currentTime && currentTime >= entry.start
                ? "rounded-2xl bg-violet-300 ring-1 ring-violet-400"
                : "",
        )}
    >
        {#if hidden}
            {new Array(entry.word.length + 1).join("_")}
        {:else}
            {entry.word}
        {/if}
    </button>
{/snippet}

{#snippet subtitleStaticArea(
    entry: SubtitleSegment["words"][0],
    hidden: boolean,
)}
    <button
        onclick={(e) => {
            e.stopPropagation();
            onClickText(entry);
        }}
        class={"tansition-all inline px-0.5 tracking-wide duration-300 ease-in-out hover:underline"}
    >
        {#if hidden}
            {new Array(entry.word.length + 1).join("_")}
        {:else}
            {entry.word}
        {/if}
    </button>
{/snippet}

<!-- prettier-ignore -->
{#snippet subtitleArea(seg: SubtitleSegment, isCurrent: boolean, hidden:boolean )}
	<div class="flex flex-row justify-between">
    {#if isCurrent }
        <div
            class={cn(
                "tansition-all m-1 flex flex-row flex-wrap gap-0.5 rounded-lg p-1 duration-100  ease-in-out",
                isCurrent
                    ? "bg-primary/20"
                    : "",
            )}
						{@attach observeVisibility(isCurrent)}
        >
            {#each seg.words as entry, index (index)}
                {@render subtitleScrollArea(entry, hidden)}
            {/each}
        </div>
    {:else}
        <div class={"m-1 flex flex-row flex-wrap gap-0.5 p-1"}>
            {#each seg.words as entry, index (index)}
                {@render subtitleStaticArea(entry, hidden)}
            {/each}
        </div>
    {/if}
		<div class="m-2">
	{#if isPlaying && isCurrent}
			<Button size="sm" variant="ghost" onclick={()=>{
				// onPlaySubtitleSegment(seg, true)
			}}>
			<Pause />
			</Button>
{:else}
			<Button size="sm" variant="ghost" onclick={()=>{
				onPlaySubtitleSegment(seg, false)
			}}>
			<Play />
			</Button>
{/if}
		</div>
	</div>

{/snippet}

<ScrollArea
    class="text-md flex max-h-80 w-full max-w-96 flex-col gap-0.5 overflow-auto bg-stone-100 px-4 py-2 tabular-nums"
>
    {#each subtitlesByText as i, index (index)}
        {@render subtitleArea(
            i,
            i.end > currentTime && currentTime >= i.start,
            true,
        )}
    {/each}
</ScrollArea>
