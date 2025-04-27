<script lang="ts">
    import { setContext } from "svelte";

    import { PlayOrYt, ActionOrYt } from "@/components/youtue";
    import YtDownloadForm from "./yt-download-form.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import { sliderValuesKey } from "@/components/youtue/yt-keys";

    import type {
        YouTubeSliderContext,
        TSLIDER_VALUES,
    } from "@/components/youtue/types";
    import Input from "@/components/ui/input/input.svelte";
    import Label from "@/components/ui/label/label.svelte";

    let sliderValues: TSLIDER_VALUES = $state([0, 100]);
    let videoUrlId = $state("dQw4w9WgXcQ123");

    setContext<YouTubeSliderContext>(sliderValuesKey, {
        getSliderValues: () => sliderValues,
    });
</script>

<div
    class="grid grid-cols-12 justify-items-stretch gap-8 overflow-hidden px-4 @container"
>
    <Card.Root class="col-span-12 @5xl:col-span-6">
        <Card.Content class="w-full">
            <YtDownloadForm bind:sliderValues />
        </Card.Content>
    </Card.Root>

    <div class="col-span-12 self-center justify-self-center @5xl:col-span-6">
        {#if videoUrlId !== "dQw4w9WgXcQ"}
            <div class="w-full">
                <Label for="videoId">Youtube Url</Label>
                <Input id="videoId" bind:value={videoUrlId} />
            </div>
        {:else}
            <PlayOrYt videoId={videoUrlId}>
                {#snippet action()}
                    <ActionOrYt bind:sliderValues />
                {/snippet}
            </PlayOrYt>
        {/if}
    </div>
</div>
