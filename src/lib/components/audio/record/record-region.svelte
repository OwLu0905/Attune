<script lang="ts">
    import { onMount } from "svelte";
    import { cn } from "@/utils";
    import Button from "@/components/ui/button/button.svelte";
    import { RecordPlayer } from "./record-player.svelte";
    import { Disc, Headphones, Mic, Save, Trash2 } from "@lucide/svelte";
    import type { RecordHistoryData } from "./record-history-data.svelte";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import NextButton from "@/components/editor/next-button.svelte";
    import { getAudioDeviceContext } from "@/audio/audioDeviceService.svelte";

    let rc: HTMLElement;
    let ws: RecordPlayer | undefined = $state(undefined);

    const audioDeviceContext = getAudioDeviceContext();

    interface Props {
        audioId: string;
        dictationId: number;
        recordData: RecordHistoryData;
        onPrev: () => void;
        onNext: () => void;
    }
    let { audioId, dictationId, recordData, onPrev, onNext }: Props = $props();

    let isSaving = $state(false);

    onMount(() => {
        ws = new RecordPlayer();
        ws.createRecord(rc);
    });

    const isRecording = $derived.by(() => {
        if (ws) return ws.isRecording;
        return false;
    });

    onMount(async () => {
        await audioDeviceContext.initializeDevices();
    });
</script>

<div class="relative flex w-full flex-col">
    <div
        bind:this={rc}
        class={cn(
            "border-border box-border w-full grow rounded-md border shadow-xs",
        )}
    ></div>

    <div class="flex shrink-0 items-center justify-center gap-2 py-2">
        <span class="text-xs tabular-nums">
            {ws?.currentTime}
        </span>

        <NextButton variant="prev" onclick={onPrev} />
        {#if isRecording}
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
        {:else if !ws?.blobData}
            <Button
                variant="ghost"
                size="sm"
                class="text-primary"
                onclick={async () => {
                    try {
                        if (!ws) return;
                        ws.createRecord(rc);

                        ws.onRecord(audioDeviceContext.selectedDeviceId);
                    } catch (error) {
                        console.error(error);
                    }
                }}
            >
                <Mic />
            </Button>
        {/if}

        {#if ws?.blobData}
            <Button
                variant="ghost"
                size="sm"
                class="text-destructive"
                onclick={() => {
                    if (!ws) return;
                    ws.deleteRecord();
                }}
            >
                <Trash2 />
            </Button>
            <Button
                disabled={isSaving}
                variant="ghost"
                size="sm"
                class="text-primary"
                onclick={async () => {
                    if (!ws) return;
                    if (!ws?.blobData || !ws?.recordedUrl) return;
                    isSaving = true;
                    await ws.onSaveFile(dictationId, audioId);

                    if (ws && ws.blobData) {
                        ws.blobData = null;
                        ws.empty();
                    }
                    isSaving = false;
                    recordData.updateData(audioId, dictationId);
                }}
            >
                <Save />
            </Button>
        {/if}

        <NextButton variant="next" onclick={onNext} />
        <DropdownMenu.Root>
            <DropdownMenu.Trigger>
                {#snippet child({ props })}
                    <Button {...props} variant="ghost" size="sm">
                        <Headphones />
                    </Button>
                {/snippet}
            </DropdownMenu.Trigger>
            <DropdownMenu.Content class="w-56">
                <DropdownMenu.Group>
                    <DropdownMenu.Label>Select Microphone</DropdownMenu.Label>
                    <DropdownMenu.Separator />
                    <DropdownMenu.RadioGroup
                        bind:value={
                            () => audioDeviceContext.selectedDeviceId,
                            (v) => {
                                audioDeviceContext.setSelectedDevice(v);
                            }
                        }
                    >
                        {#each audioDeviceContext.audioInputs ?? [] as mic}
                            <DropdownMenu.RadioItem value={mic.deviceId}>
                                {mic.label ||
                                    `Microphone ${mic.deviceId.slice(0, 8)}...`}
                            </DropdownMenu.RadioItem>
                        {/each}
                    </DropdownMenu.RadioGroup>
                </DropdownMenu.Group>
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </div>
</div>
