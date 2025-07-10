<script lang="ts">
    import { page } from "$app/state";
    import { invoke } from "@tauri-apps/api/core";
    import { getUserContext } from "@/user/userService.svelte";
    import { getAudioFile } from "@/utils";
    import AudioPlayerCard from "@/components/audio/audio-player-card.svelte";
    import type { AudioItem } from "@/types/audio";

    const { getUser } = getUserContext();
    const user = getUser();

    let audioId = $derived(page.params.id);
    let audioItem: AudioItem | undefined = $state.raw(undefined);
    let videoPath: Uint8Array | undefined = $state.raw(undefined);

    async function getAudioItem() {
        try {
            if (!user.accessToken) {
                return;
            }
            audioItem = await invoke("handle_get_audio_item", {
                token: user.accessToken,
                audio_id: audioId,
            });

            videoPath = await getAudioFile(audioId, "mp4");
        } catch (error) {
            console.error(error);
        }
    }
</script>

{#await getAudioItem()}
    <div>loading...</div>
{:then _}
    {#if audioItem && videoPath}
        <div class="relative flex h-full flex-col overflow-hidden">
            <AudioPlayerCard {audioItem} {videoPath} />
        </div>
    {/if}
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}
