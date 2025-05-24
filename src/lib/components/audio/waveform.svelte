<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    import Button from "$lib/components/ui/button/button.svelte";
    import * as Card from "$lib/components/ui/card/index.js";

    import WaveSurfer from "wavesurfer.js";
    import RegionsPlugin from "wavesurfer.js/dist/plugins/regions.esm.js";
    import TimelinePlugin from "wavesurfer.js/dist/plugins/timeline.esm.js";
    import type { Region } from "wavesurfer.js/dist/plugins/regions.esm.js";

    import { LoaderCircle, Pause, Play, Scissors } from "@lucide/svelte";
    import { WAVESURFER_BACKEND } from "$lib/constants.js";

    import TimeBadge from "$lib/components/audio/time-badge.svelte";
    import SrtRegion from "$lib/components/audio/srt-region.svelte";

    import type { SubtitleSegment } from "./types";
    import type { AudioItem } from "@/types/audio";
    import { getUserContext } from "@/user/userService.svelte";

    interface Props {
        audioPath: BlobPart;
        audioItem: AudioItem;
    }
    let { audioPath, audioItem = $bindable() }: Props = $props();

    let container: HTMLElement;
    let ws: WaveSurfer | undefined = $state(undefined);
    let activeRegion: Region | null = $state(null);
    let regions: RegionsPlugin;

    let regionList: Region[] = $state([]);
    let saveRegionList: Region[] = $state([]);
    let isEditing = $state(false);
    let removeDragSelection: (() => void) | undefined = undefined;

    let currentTime = $state(0);
    let isPlaying = $state(false);
    let isReady = $state(false);
    let isTranscribing = $state(false);

    const random = (min: number, max: number) =>
        Math.random() * (max - min) + min;
    const randomColor = () =>
        `rgba(${random(0, 255)}, ${random(0, 255)}, ${random(0, 255)}, 0.5)`;

    const { getUser } = getUserContext();
    const user = getUser();

    onMount(() => {
        const computedStyle = getComputedStyle(document.documentElement);
        const primaryHSL = computedStyle.getPropertyValue("--primary").trim();
        const secondaryHSL = computedStyle
            .getPropertyValue("--secondary")
            .trim();

        regions = RegionsPlugin.create();

        ws = WaveSurfer.create({
            container,
            progressColor: `hsl(${primaryHSL})`,
            waveColor: `hsl(${secondaryHSL})`,
            barWidth: 2,
            barGap: 1,
            height: 100,
            // backend: WAVESURFER_BACKEND,
            plugins: [regions, TimelinePlugin.create()],
        });
        const blob = new Blob([audioPath], { type: "audio/mp3" });
        const audioURL = URL.createObjectURL(blob);
        ws.load(audioURL);
        ws.setVolume(0.3);

        ws.on("decode", () => {
            if (ws === undefined) return;
            isReady = true;
            regions.on("region-created", (region) => {
                regionList.push(region);
            });

            regions.on("region-out", (region) => {
                if (activeRegion?.id === region.id) {
                    activeRegion = null;
                }
            });

            regions.on("region-updated", (region) => {
                ws!.pause();

                regionList = regionList.map((i) => {
                    if (i.id === region.id) {
                        return region;
                    }
                    return i;
                });
            });

            regions.on("region-clicked", (region, e) => {
                e.stopPropagation();
                activeRegion = region;
                region.play(true);
                region.setOptions({
                    color: randomColor(),
                });
            });

            ws.on("interaction", () => (activeRegion = null));
            ws.on("play", () => (isPlaying = true));
            ws.on("pause", () => (isPlaying = false));

            ws.on("timeupdate", (ct) => {
                currentTime = ct;
            });
        });
    });

    $effect(() => {
        if (isEditing) {
            removeDragSelection = regions.enableDragSelection({
                drag: false,
            });
        }
        if (!isEditing && removeDragSelection) {
            removeDragSelection();
        }
    });

    const onPlay = (item: Region) => {
        if (!ws) return;

        if (!activeRegion || activeRegion.id !== item.id) {
            activeRegion = item;
            item.play(true);
        } else {
            const currentTime = ws.getCurrentTime();
            const startTime =
                currentTime >= activeRegion.end
                    ? activeRegion.start
                    : currentTime;

            ws.play(startTime, activeRegion.end);
        }
    };
    const onPause = (_item: Region) => {
        ws!.pause();
    };

    const onDeleteBuffer = (item: Region) => {
        ws?.stop();
        regionList = regionList.filter((i) => {
            if (i.id !== item.id) return true;
            item.remove();
            return false;
        });
    };
    const onDeleteSaving = (item: Region) => {
        ws?.stop();
        saveRegionList = saveRegionList.filter((i) => {
            if (i.id !== item.id) return true;
            item.remove();
            return false;
        });
    };

    const onClickText = (entry: SubtitleSegment["words"][0]) => {
        ws?.pause();
        ws?.setTime(entry.start);
    };

    async function getSubtitle() {
        try {
            isTranscribing = true;

            await invoke("start_transcribe", {
                audio_id: audioItem.id,
                model: "distil-large-v3",
            });
            audioItem = await invoke("handle_update_audio_transcribe", {
                token: user.accessToken,
                audio_id: audioItem.id,
            });
        } catch (error) {
            console.error(error);
        } finally {
            isTranscribing = false;
        }
    }
</script>

<Card.Root>
    <Card.Header class="flex flex-row justify-between">
        <div class="flex flex-col gap-1.5">
            <Card.Title>{audioItem.title}</Card.Title>
            <Card.Description>{audioItem.description}</Card.Description>
        </div>
        <div class="flex items-center gap-2">
            <Button
                class="text-muted-foreground"
                type="button"
                variant="outline"
                disabled={!isReady}
                onclick={() => {
                    if (isPlaying) {
                        ws?.pause();
                    } else {
                        ws?.play();
                    }
                }}
            >
                {#if isPlaying}
                    <Pause />
                {:else}
                    <Play />
                {/if}
            </Button>
            <Button
                type="button"
                variant={isEditing ? "default" : "secondary"}
                class="flex w-40 justify-between"
                disabled={!isReady}
                onclick={() => {
                    if (isEditing) {
                        saveRegionList = [...saveRegionList, ...regionList];
                        regionList = [];
                    }
                    isEditing = !isEditing;
                }}
            >
                <Scissors />
                <div>
                    {isEditing ? "Finish Segment" : "Create Segment"}
                </div>
            </Button>
        </div>
    </Card.Header>
    <Card.Content>
        <div bind:this={container}></div>
    </Card.Content>

    <Card.Footer class="flex w-full flex-col items-start gap-2">
        {#if !isReady}
            <div>loading...</div>
        {:else if audioItem.transcribe === 0}
            <div>
                <span>no subtitle </span>
                <Button
                    disabled={isTranscribing}
                    onclick={() => {
                        getSubtitle();
                    }}
                >
                    {#if isTranscribing}
                        <LoaderCircle class="animate-spin" />
                    {/if}
                    Transcribe !
                </Button>
            </div>
        {:else}
            <SrtRegion {audioItem} {currentTime} {onClickText} {isPlaying} />
        {/if}
        <div>{currentTime.toFixed(2)}</div>
        <div class="flex gap-4">
            <div class="flex items-center gap-2">
                <div class="h-4 w-4 rounded-full bg-emerald-200"></div>
                <div>Ready to Record</div>
            </div>
            <div class="flex items-center gap-2">
                <div class="h-4 w-4 rounded-full bg-violet-200"></div>
                <div>Current Word</div>
            </div>
        </div>
    </Card.Footer>
</Card.Root>

<div class="p-6">
    <div class="flex flex-wrap gap-2.5 tabular-nums">
        <TimeBadge
            data={saveRegionList}
            onDelete={onDeleteSaving}
            {activeRegion}
            {isPlaying}
            {onPlay}
            {onPause}
        />
    </div>
</div>
