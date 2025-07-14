<script lang="ts">
    import { cn } from "@/utils";
    import { fade } from "svelte/transition";
    import { Clipboard, Eye, EyeOff, Heart, Star, Check } from "@lucide/svelte";
    import type { SubtitleSegment } from "./types";
    import type { AudioPlayer } from "./audio-player.svelte";
    import type { BookmarkDictationView } from "$lib/tauri";
    import BookOpenIcon from "./book-open-icon.svelte";
    import { writeText } from "@tauri-apps/plugin-clipboard-manager";

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

    let container: HTMLElement;
    let isCopied = $state(false);

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

<!-- prettier-ignore -->
<div
    bind:this={container}
    class={cn(
        "rounded-md px-2 py-2",
        "group relative flex  items-center",
        "before:top-0 before:-translate-x-2 before:-translate-y-4",
        !hidden && "hover:underline",
        isCurrentLine &&
            "before:text-primary before:transform-transform ease-in before:absolute before:translate-y-1.5 before:duration-300 before:content-['>']",

        isSelected &&
            "bg-gradient-to-br from-rose-100/60 via-pink-50 to-rose-100",
    )}

	{@attach (node) => {
				$effect(() => {

					if(dictationId !== index )  return 
					const observer = new IntersectionObserver(
						(entries, observer) => {
							const entry = entries[0];

							if (
								!entry.isIntersecting &&
									dictationId === index 
							) {
								node.scrollIntoView({
									behavior: "smooth",
									block: "center",
								});
								observer.unobserve(node);
							}
						},
						{
							threshold: 1.0,
							rootMargin: "120px",
						},
					);
					observer.observe(node);
					return () => {

						observer.disconnect();
					}
				});
	}}
>
    <div class={cn("ml-2 flex shrink items-start gap-2")}>
        <div class="mx-0.5 flex items-center gap-1.5">
            <button
                onclick={() => {
                    container.scrollIntoView({
                        behavior: "instant",
                        block: "center",
                    });
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
                        <Star
                            class="h-6 w-5 fill-yellow-300 stroke-amber-300 stroke-1"
                        />
                    </button>
                </div>
            {:else}
                <div in:fade class="h-6 w-5">
                    <button
                        onclick={() => {
                            createBookmarkItem(index);
                        }}
                    >
                        <Star class="h-6 w-5 stroke-1 opacity-30" />
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

            <div
                class={cn(
                    "flex shrink-0 gap-2 pl-4",
                    isCopied ? "opacity-100" : "opacity-0 group-hover:opacity-100"
                )}
            >
                {#if hidden}
                    <Eye
                        class="stoke-1 h-6 w-5"
                        onclick={(e) => {
                            e.stopPropagation();
                            hidden = false;
                        }}
                    />
                {:else}
                    <EyeOff
                        class="opacity-80 stoke-1 h-6 w-5"
                        onclick={(e) => {
                            e.stopPropagation();
                            hidden = true;
                        }}
                    />
                {/if}
								<button
									onclick={async()=>{
										try {
											await writeText(segment.text);
											isCopied = true;
											setTimeout(() => {
												isCopied = false;
											}, 1000);
										} catch (error) {
											throw new Error("clip failed")
										}
									}}
								>
									{#if isCopied}
										<div in:fade>
											<Check class="h-4 w-4 text-green-500" />
										</div>
									{:else}
										<Clipboard class="h-4 w-4" />
									{/if}
								</button>
            </div>
        </div>
    </div>
</div>
