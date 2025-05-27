<script lang="ts">
    import { page } from "$app/state";
    import { invoke } from "@tauri-apps/api/core";
    import { getUserContext } from "@/user/userService.svelte";
    import { getAudioFile } from "@/utils";

    import Waveform from "@/components/audio/waveform.svelte";

    import type { Attachment } from "svelte/attachments";
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
    <div class="relative flex flex-col overflow-hidden">
        <div class="flex h-full w-full shrink grow flex-col overflow-scroll">
            <Waveform {audioItem} {audioPath} />
        </div>
    </div>
{/if}
