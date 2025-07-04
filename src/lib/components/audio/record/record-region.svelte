<script lang="ts">
    import { onMount } from "svelte";
    import { fade, fly } from "svelte/transition";
    import { cn } from "@/utils";
    import Button from "@/components/ui/button/button.svelte";
    import { RecordPlayer } from "./record-player.svelte";
    import RecordPlugin from "wavesurfer.js/dist/plugins/record.esm.js";
    import { Disc, Mic, Save } from "@lucide/svelte";
    import type { RecordHistoryData } from "./record-history-data.svelte";

    let rc: HTMLElement;
    let ws: RecordPlayer | undefined = $state(undefined);
    let micSelect: MediaDeviceInfo[] = $state([]);

    interface Props {
        audioId: string;
        questionId: string;
        recordData: RecordHistoryData;
    }
    let { audioId, questionId, recordData }: Props = $props();

    let isSaving = $state(false);

    onMount(() => {
        ws = new RecordPlayer();
        ws.createRecord(rc);
    });

    const isRecording = $derived.by(() => {
        if (ws) return ws.isRecording;
        return false;
    });

    onMount(() => {
        async function getAudioDevices() {
            RecordPlugin.getAvailableAudioDevices().then((devices) => {
                micSelect = devices;
            });
        }
        getAudioDevices();
    });

    const triggerContent = $derived(micSelect[0]?.label ?? "Select mic");
</script>

<div class="relative flex w-full">
    <div
        bind:this={rc}
        class={cn(
            "border-border box-border w-full rounded-md border shadow-xs",
        )}
    ></div>

    <div class="z-5 flex w-60 items-center justify-end gap-2 py-2">
        {#if isRecording}
            <div in:fade>
                <Button
                    variant="ghost"
                    size="sm"
                    class="text-destructive"
                    onclick={() => {
                        if (!ws) return;
                        ws.onFinish();
                    }}
                >
                    <Disc />
                </Button>
            </div>
        {:else}
            <div in:fade>
                <Button
                    variant="ghost"
                    size="sm"
                    class="text-primary"
                    onclick={async () => {
                        try {
                            if (!ws) return;
                            ws.createRecord(rc);

                            ws.onRecord(micSelect[0]?.deviceId ?? "");
                        } catch (error) {
                            console.error(error);
                        }
                    }}
                >
                    <Mic />
                </Button>
            </div>
        {/if}

        {#if ws?.blobData}
            <div out:fly={{ y: -20, duration: 600 }}>
                <Button
                    disabled={isSaving}
                    variant="ghost"
                    size="sm"
                    class="text-primary"
                    onclick={async () => {
                        if (!ws) return;
                        if (!ws?.blobData || !ws?.recordedUrl) return;
                        isSaving = true;
                        await ws.onSaveFile(questionId, audioId);

                        if (ws && ws.blobData) {
                            ws.blobData = null;
                            ws.empty();
                        }
                        isSaving = false;
                        recordData.updateData(audioId, questionId);
                    }}
                >
                    <Save />
                </Button>
            </div>
        {/if}
    </div>
</div>
