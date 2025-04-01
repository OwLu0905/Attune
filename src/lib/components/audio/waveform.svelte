<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import { onMount } from "svelte";

    import WaveSurfer from "wavesurfer.js";
    import RegionsPlugin from "wavesurfer.js/dist/plugins/regions.esm.js";
    import TimelinePlugin from "wavesurfer.js/dist/plugins/timeline.esm.js";
    import type { Region } from "wavesurfer.js/dist/plugins/regions.esm.js";

    import { Play, Scissors } from "@lucide/svelte";
    import { WAVESURFER_BACKEND } from "$lib/constants.js";

    import TimeBadge from "$lib/components/audio/time-badge.svelte";
    import SrtRegion from "$lib/components/audio/srt-region.svelte";
    import type { SubtitleEntity } from "$lib/components/audio/srt-region.svelte";

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

    const random = (min: number, max: number) =>
        Math.random() * (max - min) + min;
    const randomColor = () =>
        `rgba(${random(0, 255)}, ${random(0, 255)}, ${random(0, 255)}, 0.5)`;

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
            url: "/audio.wav",
            height: 100,
            backend: WAVESURFER_BACKEND,
            plugins: [regions, TimelinePlugin.create()],
        });

        ws.on("decode", () => {
            if (ws === undefined) return;

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
                currentTime = ct * 1000;
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

    const onClickText = (entry: SubtitleEntity) => {
        ws?.pause();
        ws?.setTime(entry.startTime / 1000);
    };
</script>

<Card.Root>
    <Card.Header class="flex flex-row justify-between">
        <div class="flex flex-col gap-1.5">
            <Card.Title>Card Title</Card.Title>
            <Card.Description>Card Description</Card.Description>
        </div>
        <div class="flex items-center gap-2">
            <Button
                class="text-muted-foreground"
                type="button"
                variant="outline"
                onclick={() => {
                    ws?.play();
                }}
            >
                <Play />
            </Button>
            <Button
                type="button"
                variant={isEditing ? "default" : "secondary"}
                class="flex w-40 justify-between"
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
    <Card.Footer class="flex w-full flex-col gap-2">
        <SrtRegion {currentTime} {onClickText} />
        <div>{(currentTime / 1000).toFixed(2)}</div>
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
