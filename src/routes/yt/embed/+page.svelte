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

<div class="flex gap-12 px-16">
    <Card.Root class="flex-1">
        <Card.Content>
            <YtDownloadForm bind:sliderValues />
        </Card.Content>
    </Card.Root>

    <div
        class="flex flex-shrink flex-col gap-2 rounded-md bg-primary-foreground"
    >
        <PlayOrYt videoId={videos[0].id}>
            {#snippet action()}
                <ActionOrYt bind:sliderValues />
            {/snippet}
        </PlayOrYt>
    </div>
</div>
