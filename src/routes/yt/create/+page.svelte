<script lang="ts">
    import { fade } from "svelte/transition";
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
    let errMsg: null | string = $state(null);

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
        errMsg = null;
        const data = new FormData(e.currentTarget);
        const videoUrl = Object.fromEntries(data).videoUrl;
        const urlParse = z.string().url();

        try {
            const resultUrl = urlParse.parse(videoUrl);
            const parseUrl = new URL(resultUrl);
            const videoId = parseUrl.searchParams.get("v");

            if (!videoId) {
                throw new Error("Can't get videoId");
            }

            const ytEmbedResult = await getYtOembed(resultUrl);

            if (!ytEmbedResult) {
                throw new Error("Can't get youtube embed result");
            }

            urlInfo = {
                url: resultUrl,
                embedInfo: { ...ytEmbedResult },
            };
            videoUrlId = videoId;
        } catch (error) {
            if (error instanceof z.ZodError) {
                errMsg = error.errors.map((err) => err.message).join(", ");
            } else if (error instanceof Error) {
                errMsg = error.message;
            } else {
                console.error("An unknown error occurred:", String(error));
                errMsg = "An unknown error occurred";
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
    class="@container grid h-full grid-cols-12 justify-items-stretch gap-8 overflow-hidden px-4"
>
    {#if videoUrlId !== "" && urlInfo !== null}
        <div in:fade class="col-span-12 @4xl:col-span-6">
            <Card.Root class="col-span-12 @4xl:col-span-6">
                <Card.Content class="w-full">
                    <YtDownloadForm bind:sliderValues bind:urlInfo />
                </Card.Content>
            </Card.Root>
        </div>
    {/if}

    {#if videoUrlId === "" || urlInfo === null}
        <div class="col-span-8 col-start-3 mt-40 items-stretch" in:fade>
            <div class="my-2">
                <form class="flex flex-col gap-2" onsubmit={handleSubmit}>
                    <Label
                        for="videoId"
                        class="data-error:text-destructive"
                        data-error={errMsg}>Youtube Url</Label
                    >
                    <div class="flex gap-2">
                        <Input id="videoId" name="videoUrl" />
                        <Button type="submit" disabled={checking}>
                            {#if checking}
                                <LoaderCircle class="animate-spin" />
                            {/if}
                            Load</Button
                        >
                    </div>
                    <span class="text-destructive text-xs">{errMsg}</span>
                </form>
            </div>
        </div>
    {:else}
        <div class="col-span-12 @4xl:col-span-6" in:fade>
            <PlayOrYt videoId={videoUrlId}>
                <Button
                    class="absolute top-0 right-0"
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
        </div>
    {/if}
</div>
