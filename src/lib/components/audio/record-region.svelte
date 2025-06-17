<script lang="ts">
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";
    import { cn } from "@/utils";
    import Button from "@/components/ui/button/button.svelte";
    import { RecordPlayer } from "./record-player.svelte";
    import RecordPlugin from "wavesurfer.js/dist/plugins/record.esm.js";
    import { Disc, Mic } from "@lucide/svelte";

    let rc: HTMLElement;
    let ws: RecordPlayer | undefined = $state(undefined);
    let micSelect: any[] = $state([]);

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

<div class="relative flex w-full flex-col">
    <div
        bind:this={rc}
        class={cn(
            "border-border box-border w-full rounded-md border shadow-xs",
        )}
    ></div>

    <div class="z-5 flex w-full items-center justify-center gap-2 py-2">
        {#if isRecording}
            <div in:fade>
                <Button
                    variant="secondary"
                    size="icon"
                    class="text-destructive"
                    onclick={() => {
                        if (!ws) return;
                        ws.onFinish();
                    }}
                >
                    <Disc strokeWidth={1.5} />
                </Button>
            </div>
        {:else}
            <div in:fade>
                <Button
                    variant="secondary"
                    size="icon"
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
                    <Mic strokeWidth={1.5} />
                </Button>
            </div>
        {/if}
    </div>
</div>
