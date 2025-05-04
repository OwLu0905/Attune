<script lang="ts">
    import { getContext } from "svelte";

    import { Play, Pause } from "@lucide/svelte";

    import Slider from "@/components/ui/slider/slider.svelte";
    import Progress from "@/components/ui/progress/progress.svelte";
    import Button from "@/components/ui/button/button.svelte";

    import { simpleFormatSecondsToMMSS } from "@/utils";

    import { ytKey } from "./yt-keys";
    import type { TSLIDER_VALUES, YouTubePlayerContext } from "./types";

    interface Props {
        sliderValues: TSLIDER_VALUES;
    }

    // TODO: fix the play button:
    // the current version will replay the player
    let { sliderValues = $bindable() }: Props = $props();

    const { getPlayer, getReady } = getContext<YouTubePlayerContext>(ytKey);

    let player = $derived(getPlayer());
    let isReady = $derived(getReady());

    let duration = $derived(player?.getDuration());
    let progressValue = $state(0);

    let playState = $state(YT.PlayerState.UNSTARTED);

    $effect(() => {
        if (!player) return;

        let timer = 0;
        const playChange = (event: YT.OnStateChangeEvent) => {
            playState = event.data;
            if (event.data === YT.PlayerState.PLAYING) {
                timer = setInterval(() => {
                    progressValue = player.getCurrentTime();
                    if (player.getCurrentTime() >= sliderValues[1]) {
                        player.pauseVideo();
                    }
                }, 100);
            }
        };
        player.addEventListener("onStateChange", playChange);

        return () => {
            console.log("remove");
            player.removeEventListener("onStateChange", playChange);
            if (timer) {
                clearInterval(timer);
            }
        };
    });

    function seekTo(allowSeekAhead: boolean = false) {
        player?.seekTo(10, allowSeekAhead);
    }

    function playeVideo() {
        player?.seekTo(sliderValues[0], true);
        if (playState !== YT.PlayerState.PLAYING) {
            player?.playVideo();
        }
    }

    function pauseVideo() {
        player?.pauseVideo();
    }
</script>

<div class="flex w-full flex-col">
    <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-1">
            <div
                class="flex items-center justify-end text-xs font-light tabular-nums"
            >
                <span
                    >{simpleFormatSecondsToMMSS(
                        Math.floor(progressValue),
                    )}</span
                >
            </div>
            <Progress
                class="h-3 rounded-md"
                value={Math.floor((progressValue * 100) / (duration ?? 1))}
            />
        </div>
        <Slider
            type="multiple"
            bind:value={sliderValues}
            max={duration}
            step={1}
            onValueCommit={(v) => {
                pauseVideo();
            }}
        />
    </div>
    <div class="flex justify-center gap-2">
        {#if playState !== YT.PlayerState.PLAYING}
            <Button
                class=""
                type="button"
                variant="ghost"
                onclick={playeVideo}
                disabled={!isReady}
            >
                <Play />
            </Button>
        {/if}

        {#if playState === YT.PlayerState.PLAYING}
            <Button
                class=""
                type="button"
                variant="ghost"
                onclick={pauseVideo}
                disabled={!isReady}
            >
                <Pause />
            </Button>
        {/if}
    </div>
</div>
