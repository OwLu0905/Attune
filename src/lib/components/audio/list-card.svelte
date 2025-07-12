<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { getUserContext } from "@/user/userService.svelte";
    import { fade } from "svelte/transition";

    import ScrollArea from "@/components/ui/scroll-area/scroll-area.svelte";
    import SegmentField from "./segment-field.svelte";
    import Error from "../error/error.svelte";
    import { Skeleton } from "@/components/ui/skeleton";

    import { Eye, EyeOff, Settings } from "@lucide/svelte";

    import type { AudioItem } from "@/types/audio";
    import type { SubtitleSegment } from "./types";
    import type { AudioPlayer } from "./audio-player.svelte";

    export type TBookmarkItem = { bookmarkId: number; createdAt: string };

    interface Props {
        audioItem: AudioItem;
        subtitles: SubtitleSegment[];
        audioPlayer: AudioPlayer | undefined;
        onPause: () => Promise<void>;
        onPlaySection: (start: number, end: number) => Promise<void>;
        dictationId: number;
    }
    let {
        audioItem,
        subtitles,
        audioPlayer,
        onPause,
        onPlaySection,

        dictationId = $bindable(),
    }: Props = $props();

    let hidden = $state(true);

    const { getUser } = getUserContext();
    const user = getUser();
    let bookmarkList = $state<TBookmarkItem[]>([]);

    async function getBookmarkList() {
        try {
            bookmarkList = await invoke("handle_get_bookmark_list", {
                token: user.accessToken,
                audio_id: audioItem.id,
            });
        } catch (error) {
            console.error(error);
        }
    }
    async function createBookmarkItem(index: number) {
        try {
            bookmarkList = await invoke("handle_create_bookmark_item", {
                token: user.accessToken,
                audio_id: audioItem.id,
                bookmark_id: index,
            });
        } catch (error) {
            console.error(error);
        }
    }
    async function deleteBookmarkItem(index: number) {
        try {
            bookmarkList = await invoke("handle_delete_bookmark_item", {
                token: user.accessToken,
                audio_id: audioItem.id,
                bookmark_id: index,
            });
        } catch (error) {
            console.error(error);
        }
    }

    function getDictation(index: number) {
        dictationId = index;
    }
</script>

{#if audioPlayer}
    <div
        transition:fade
        class="relative h-full shrink grow overflow-auto bg-white ring"
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
        {#await getBookmarkList() then _}
            <ScrollArea class="px-4 tabular-nums">
                <div class="mx-2 flex flex-col gap-4.5">
                    {#each subtitles as segment, index (index)}
                        <SegmentField
                            {audioPlayer}
                            {segment}
                            {hidden}
                            {onPause}
                            {onPlaySection}
                            {index}
                            {getDictation}
                            {bookmarkList}
                            {createBookmarkItem}
                            {deleteBookmarkItem}
                        />
                    {/each}
                </div>
            </ScrollArea>
        {:catch}
            <Error />
        {/await}
    </div>
{:else}
    <Skeleton class="h-80 w-full rounded-xl" />
{/if}
