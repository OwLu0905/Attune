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

    let sliderValues: TSLIDER_VALUES = $state([0, 100]);

    setContext<YouTubeSliderContext>(sliderValuesKey, {
        getSliderValues: () => sliderValues,
    });

    let videos = $state([
        {
            id: "dQw4w9WgXcQ",
            title: "Rick Astley - Never Gonna Give You Up",
        },
    ]);
</script>

<div class="grid grid-cols-12 gap-8 overflow-hidden px-4 @container">
    <Card.Root class="col-span-12 @5xl:col-span-6">
        <Card.Content class="w-full">
            <YtDownloadForm bind:sliderValues />
        </Card.Content>
    </Card.Root>

    <div class="col-span-12 w-full @5xl:col-span-6">
        <PlayOrYt videoId={videos[0].id}>
            {#snippet action()}
                <ActionOrYt bind:sliderValues />
            {/snippet}
        </PlayOrYt>
    </div>
</div>
