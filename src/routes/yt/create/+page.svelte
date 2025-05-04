<script lang="ts">
    import { setContext } from "svelte";

    import { z } from "zod";
    import { LoaderCircle, X } from "@lucide/svelte";

    import * as Card from "$lib/components/ui/card/index.js";
    import Input from "@/components/ui/input/input.svelte";
    import Label from "@/components/ui/label/label.svelte";
    import Button from "@/components/ui/button/button.svelte";

    import YtDownloadForm from "./yt-download-form.svelte";
    import { getYtOembed } from "./utils";
    import { PlayOrYt, ActionOrYt } from "@/components/youtue";

    import { sliderValuesKey } from "@/components/youtue/yt-keys";

    import type {
        YouTubeSliderContext,
        TSLIDER_VALUES,
    } from "@/components/youtue/types";
    import type { YtOembUrlInfo } from "./types";

    let sliderValues: TSLIDER_VALUES = $state([0, 100]);
    let videoUrlId = $state("");
    let urlInfo: YtOembUrlInfo | null = $state.raw(null);
    let checking = $state(false);

    setContext<YouTubeSliderContext>(sliderValuesKey, {
        getSliderValues: () => sliderValues,
    });

    async function handleSubmit(
        e: SubmitEvent & {
            currentTarget: EventTarget & HTMLFormElement;
        },
    ) {
        e.preventDefault();

        checking = true;
        const data = new FormData(e.currentTarget);
        const videoUrl = Object.fromEntries(data).videoUrl;
        const urlParse = z.string().url();

        try {
            const resultUrl = urlParse.parse(videoUrl);
            const ytEmbedResult = await getYtOembed(resultUrl);
            const parseUrl = new URL(resultUrl);
            const videoId = parseUrl.searchParams.get("v");

            if (!videoId || !ytEmbedResult) {
                throw new Error("Can't get videoId or youtube embed result");
            }

            urlInfo = {
                url: resultUrl,
                embedInfo: { ...ytEmbedResult },
            };
            videoUrlId = videoId;
        } catch (error) {
            if (error instanceof z.ZodError) {
                console.error("Invalid URL format:", error.format());
            } else if (error instanceof Error) {
                console.error(error.message);
            } else {
                console.error("An unknown error occurred:", String(error));
            }
        } finally {
            checking = false;
        }
    }

    function handleStateReset() {
        videoUrlId = "";
        urlInfo = null;
        sliderValues = [0, 100];
    }
</script>

<div
    class="grid grid-cols-12 justify-items-stretch gap-8 overflow-hidden px-4 @container"
>
    <Card.Root class="col-span-12 @5xl:col-span-6">
        <Card.Content class="w-full">
            <YtDownloadForm bind:sliderValues bind:urlInfo />
        </Card.Content>
    </Card.Root>

    <div class="col-span-12 self-center @5xl:col-span-6">
        {#if videoUrlId === "" || urlInfo === null}
            <div class="my-2">
                <form class="flex flex-col gap-2" onsubmit={handleSubmit}>
                    <Label for="videoId">Youtube Url</Label>
                    <div class="flex gap-2">
                        <Input id="videoId" name="videoUrl" />
                        <Button type="submit" disabled={checking}>
                            {#if checking}
                                <LoaderCircle class="animate-spin" />
                            {/if}
                            Load</Button
                        >
                    </div>
                </form>
            </div>
        {:else}
            <PlayOrYt videoId={videoUrlId}>
                <Button
                    class="absolute right-0 top-0"
                    onclick={() => {
                        handleStateReset();
                    }}
                >
                    <X />
                </Button>
                {#snippet action()}
                    <ActionOrYt bind:sliderValues />
                {/snippet}
            </PlayOrYt>
        {/if}
    </div>
</div>
