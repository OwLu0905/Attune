<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import { onMount } from "svelte";

    import WaveSurfer from "wavesurfer.js";
    import RegionsPlugin from "wavesurfer.js/dist/plugins/regions.esm.js";
    import type { Region } from "wavesurfer.js/dist/plugins/regions.esm.js";

    import { Play, Scissors } from "@lucide/svelte";
    import { WAVESURFER_BACKEND } from "$lib/constants.js";
    import TimeBadge from "./time-badge.svelte";

    let container: HTMLElement;
    let ws: WaveSurfer | undefined = $state(undefined);
    let activeRegion: Region | null = $state(null);
    let regions: RegionsPlugin;

    let regionList: Region[] = $state([]);
    let saveRegionList: Region[] = $state([]);
    let isEditing = $state(false);
    let removeDragSelection: (() => void) | undefined = undefined;

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
            barWidth: 4,
            barGap: 2,
            url: "/audio.wav",
            backend: WAVESURFER_BACKEND,
            plugins: [regions],
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
    <Card.Footer class="flex flex-wrap gap-2">
        <div class="flex flex-wrap gap-2.5 tabular-nums">
            <TimeBadge
                data={regionList}
                onDelete={onDeleteBuffer}
                {activeRegion}
                {isPlaying}
                {onPlay}
                {onPause}
            />
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
