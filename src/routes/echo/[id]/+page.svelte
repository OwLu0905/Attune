<script lang="ts">
    import { page } from "$app/state";
    import { invoke } from "@tauri-apps/api/core";
    import { readFile, BaseDirectory } from "@tauri-apps/plugin-fs";
    import { getUserContext } from "@/user/userService.svelte";
    import { getAudioSubtitlePath } from "@/utils";

    import Waveform from "@/components/audio/waveform.svelte";
    import type { AudioItem } from "@/types/audio";

    const { getUser } = getUserContext();

    const user = getUser();

    let audioId = $derived(page.params.id);

    let audioItem: AudioItem | undefined = $state(undefined);

    let audioPath: Uint8Array | undefined = $state(undefined);

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

                const path = await getAudioSubtitlePath(audioId);

                if (!path) return;

                const audioUrl = path[0];

                audioPath = await readFile(`data/${audioUrl}/audio.mp3`, {
                    baseDir: BaseDirectory.AppLocalData,
                });
            } catch (error) {
                console.error(error);
            }
        }

        getAudioItem();
    });

    $inspect(audioPath);
</script>

{#if audioPath === undefined || !audioItem}
    <div>no data</div>
{:else}
    <div class="relative flex flex-col overflow-hidden">
        <div class="flex h-full w-full shrink grow flex-col overflow-scroll">
            <Waveform subTitle={audioItem!.transcribe} {audioPath} />
        </div>
    </div>
{/if}
