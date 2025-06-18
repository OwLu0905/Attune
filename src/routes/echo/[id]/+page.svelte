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
    let audioPath: Uint8Array | undefined = $state.raw(undefined);

    $effect(() => {
        async function getAudioItem() {
            try {
                if (!user.accessToken) {
                    return;
                }
                audioItem = await invoke("handle_get_audio_item", {
                    token: user.accessToken,
                    audio_id: audioId,
                });

                audioPath = await getAudioFile(audioId);
            } catch (error) {
                console.error(error);
            }
        }

        getAudioItem();
    });
</script>

{#if audioPath === undefined || !audioItem}
    <div>no data</div>
{:else}
    <div class="relative flex h-full flex-col overflow-hidden">
        <AudioPlayerCard {audioItem} {audioPath} />
    </div>
{/if}
