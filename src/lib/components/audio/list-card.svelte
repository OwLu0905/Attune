<script lang="ts">
    import { fade } from "svelte/transition";
    import { commands } from "$lib/tauri";
    import { getUserContext } from "@/user/userService.svelte";

    import ScrollArea from "@/components/ui/scroll-area/scroll-area.svelte";
    import SegmentField from "./segment-field.svelte";
    import ErrorMessage from "../error/error-message.svelte";
    import { Skeleton } from "@/components/ui/skeleton";

    import { Eye, EyeOff, Settings } from "@lucide/svelte";

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
</script>

{#if audioPlayer}
    <div
        transition:fade
        class="bg-background border-input relative h-full shrink grow overflow-auto rounded-md border-1"
    >
        <div class="sticky top-0 z-5 flex w-full gap-2 bg-inherit px-4 py-2">
            <Settings size={16} />
            {#if hidden}
                <Eye
                    class="h-4 w-4"
                    onclick={() => {
                        hidden = false;
                    }}
                />
            {:else}
                <EyeOff
                    class="h-4 w-4"
                    onclick={() => {
                        hidden = true;
                    }}
                />
            {/if}
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
