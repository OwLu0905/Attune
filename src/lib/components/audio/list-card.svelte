<script lang="ts">
    import { fade } from "svelte/transition";
    import { commands } from "$lib/tauri";
    import { getUserContext } from "@/user/userService.svelte";

    import Checkbox from "@/components/ui/checkbox/checkbox.svelte";
    import Label from "@/components/ui/label/label.svelte";
    import Button from "@/components/ui/button/button.svelte";
    import ScrollArea from "@/components/ui/scroll-area/scroll-area.svelte";
    import { Skeleton } from "@/components/ui/skeleton";

    import SegmentField from "./segment-field.svelte";
    import ErrorMessage from "../error/error-message.svelte";

    import { ChevronsDownUp, Eye, EyeOff } from "@lucide/svelte";

    import type { AudioItem } from "$lib/tauri";
    import type { SubtitleSegment } from "./types";
    import type { AudioPlayer } from "./audio-player.svelte";
    import type { BookmarkDictationView } from "$lib/tauri";

    interface Props {
        dictationId: number;
        combinedList: BookmarkDictationView[];
        audioItem: AudioItem;
        subtitles: SubtitleSegment[];
        audioPlayer: AudioPlayer | undefined;
        onPause: () => Promise<void>;
        onPlaySection: (
            start: number,
            end: number,
            setEnd?: boolean,
        ) => Promise<void>;
    }
    let {
        audioItem,
        subtitles,
        audioPlayer,
        onPause,
        onPlaySection,
        dictationId = $bindable(),
        combinedList = $bindable(),
    }: Props = $props();

    // TODO: lift this to parent and pass into dictation-editor
    let hidden = $state(true);
    let autoScroll = $state(false);

    const { getUser } = getUserContext();
    const user = getUser();

    async function getCombinedList() {
        if (!user.accessToken) return;

        try {
            const result = await commands.handleGetBookmarkDictationCombined(
                user.accessToken,
                audioItem.id,
            );

            if (result.status === "error") {
                throw new Error(result.error);
            }

            combinedList = result.data;
            const latestId =
                combinedList[combinedList.length - 1].dictationPosition;

            dictationId = latestId ?? 0;
        } catch (error) {
            console.error(error);
        }
    }
    async function createBookmarkItem(index: number) {
        if (!user.accessToken) return;

        try {
            const result = await commands.handleCreateBookmarkItem(
                user.accessToken,
                audioItem.id,
                index,
            );

            if (result.status === "error") {
                throw new Error(result.error);
            }

            combinedList = result.data;
        } catch (error) {
            console.error(error);
        }
    }
    async function deleteBookmarkItem(index: number) {
        if (!user.accessToken) return;

        try {
            const result = await commands.handleDeleteBookmarkItem(
                user.accessToken,
                audioItem.id,
                index,
            );

            if (result.status === "error") {
                throw new Error(result.error);
            }

            combinedList = result.data;
        } catch (error) {
            console.error(error);
        }
    }

    function getDictation(index: number, startTime: number) {
        dictationId = index;
        audioPlayer?.onPause();
        audioPlayer?.onSetTime(startTime);
    }

    function scrollToCurrent(list: SubtitleSegment[]) {
        if (!audioPlayer) return;

        const id = list.findIndex((i) => {
            return (
                i.start <= audioPlayer.currentTime &&
                audioPlayer.currentTime <= i.end
            );
        });
        if (id === dictationId) {
            // TODO:
            dictationId = -1;
            setTimeout(() => {
                dictationId = id;
            }, 0);
        } else {
            dictationId = id;
        }
    }

    $effect(() => {
        let frameId: number;

        function scrollToCurrent(list: SubtitleSegment[]) {
            if (!audioPlayer) return;

            const id = list.findIndex((i) => {
                return (
                    i.start <= audioPlayer.currentTime &&
                    audioPlayer.currentTime <= i.end
                );
            });

            if (id > -1) {
                dictationId = id;
            }

            frameId = requestAnimationFrame(() => {
                scrollToCurrent(list);
            });
        }

        if (autoScroll && audioPlayer?.isPlaying) {
            scrollToCurrent(subtitles);
        }

        return () => {
            if (frameId) {
                cancelAnimationFrame(frameId);
            }
        };
    });
</script>

{#snippet scrollToArea(list: SubtitleSegment[])}
    <Button
        size="sm"
        variant="ghost"
        onclick={() => {
            scrollToCurrent(list);
        }}
        class="group"
    >
        <ChevronsDownUp
            class="size-3.5 transition-transform duration-150 ease-in group-active:scale-125"
        />
    </Button>
{/snippet}

{#if audioPlayer}
    <div
        transition:fade
        class="bg-background border-input relative h-full shrink grow overflow-auto rounded-md border-1"
    >
        <div
            class="sticky top-0 z-5 flex w-full items-center gap-1 bg-inherit px-4 py-2"
        >
            {#if hidden}
                <Button
                    size="sm"
                    variant="ghost"
                    onclick={() => {
                        hidden = false;
                    }}
                    class="group"
                >
                    <Eye
                        class="size-3.5 transition-transform duration-100 ease-in group-active:scale-110"
                    />
                </Button>
            {:else}
                <Button
                    size="sm"
                    variant="ghost"
                    onclick={() => {
                        hidden = true;
                    }}
                    class="group"
                >
                    <EyeOff
                        class="size-3.5 transition-transform duration-100 ease-in group-active:scale-110"
                    />
                </Button>
            {/if}

            {@render scrollToArea(subtitles)}
            <div class="ml-auto">
                <div class="flex items-center space-x-2">
                    <Checkbox bind:checked={autoScroll} id="auto-scroll" />
                    <Label for="auto-scroll">Auto Scroll</Label>
                </div>
            </div>
        </div>
        {#await getCombinedList() then _}
            <ScrollArea class="px-4 pb-2 tabular-nums">
                <div class="flex flex-col gap-2.5">
                    {#each subtitles as segment, index (index)}
                        <SegmentField
                            {audioPlayer}
                            {segment}
                            {hidden}
                            {onPause}
                            {onPlaySection}
                            {index}
                            {dictationId}
                            {getDictation}
                            {combinedList}
                            {createBookmarkItem}
                            {deleteBookmarkItem}
                        />
                    {/each}
                </div>
            </ScrollArea>
        {:catch}
            <ErrorMessage />
        {/await}
    </div>
{:else}
    <Skeleton class="h-80 w-full rounded-xl" />
{/if}
