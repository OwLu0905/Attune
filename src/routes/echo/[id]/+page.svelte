<script lang="ts">
    import { page } from "$app/state";
    import { commands } from "$lib/tauri";
    import { getUserContext } from "@/user/userService.svelte";
    import { getAudioFile } from "@/utils";
    import AudioPlayerCard from "@/components/audio/audio-player-card.svelte";
    import type { AudioListItem } from "$lib/tauri";
    import { fade } from "svelte/transition";

    const { getUser } = getUserContext();
    const user = getUser();

    let audioId = $derived(page.params.id);
    let audioItem: AudioListItem | undefined = $state.raw(undefined);
    let videoPath: Uint8Array<ArrayBuffer> | undefined = $state.raw(undefined);

    async function getAudioItem() {
        try {
            if (!user.accessToken) {
                return;
            }
            const result = await commands.handleGetAudioItem(
                user.accessToken,
                audioId,
            );

            if (result.status === "error") {
                throw new Error(result.error);
            }

            if (result.data) {
                audioItem = result.data;
            }

            videoPath = await getAudioFile(audioId, "mp4");
        } catch (error) {
            console.error(error);
        }
    }
</script>

{#await getAudioItem()}
    <div class="relative flex h-full w-full flex-col overflow-hidden">
        <div in:fade class="h-full w-full"></div>
    </div>
{:then _}
    {#if audioItem && videoPath}
        <div class="relative flex h-full flex-col overflow-hidden">
            <AudioPlayerCard {audioItem} {videoPath} />
        </div>
    {/if}
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}
