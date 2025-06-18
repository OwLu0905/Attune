<script lang="ts">
    import { onDestroy } from "svelte";
    import { fade } from "svelte/transition";

    import type { RecordHistoryData } from "./record-history-data.svelte";
    import RecordAudioPlayer from "./record-audio-player.svelte";

    interface Props {
        data: Uint8Array;
        audioId: string;
        questionId: string;
        name: string;
        recordData: RecordHistoryData;
    }
    let { audioId, questionId, data, name, recordData }: Props = $props();

    let url = $derived.by(() => {
        const blob = new Blob([data], {
            type: "audio/webm",
        }); // or appropriate MIME type
        const audioUrl = URL.createObjectURL(blob);
        return audioUrl;
    });

    onDestroy(() => {
        URL.revokeObjectURL(url);
    });
</script>

<div
    out:fade
    class="bg-card border-foreground/20 text-foreground hover:border-primary/20 w-1/4 gap-1 rounded-xl border text-xs transition-all duration-150 ease-linear"
>
    <RecordAudioPlayer
        src={url}
        title={name}
        onDelete={() => {
            recordData.deleteData(audioId, questionId, name);
        }}
    />
</div>
