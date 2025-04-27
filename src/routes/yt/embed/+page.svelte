<script lang="ts">
    import { setContext } from "svelte";

    import * as Card from "$lib/components/ui/card/index.js";
    import Input from "@/components/ui/input/input.svelte";
    import Label from "@/components/ui/label/label.svelte";
    import Button from "@/components/ui/button/button.svelte";

    import { PlayOrYt, ActionOrYt } from "@/components/youtue";
    import YtDownloadForm from "./yt-download-form.svelte";
    import { sliderValuesKey } from "@/components/youtue/yt-keys";

    import type {
        YouTubeSliderContext,
        TSLIDER_VALUES,
    } from "@/components/youtue/types";

    let sliderValues: TSLIDER_VALUES = $state([0, 100]);

    let videoUrlId = $state("");

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
                <form
                    class="flex flex-col gap-2"
                    onsubmit={(e) => {
                        e.preventDefault();
                        if (e.currentTarget) {
                            const data = new FormData(e.currentTarget);

                            console.log(data);
                        }
                    }}
                >
                    <Label for="videoId">Youtube Url</Label>
                    <div class="flex gap-2">
                        <Input id="videoId" name="videoUrl" />
                        <Button type="submit">Load</Button>
                    </div>
                </form>
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
