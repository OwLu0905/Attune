<script lang="ts">
    import Button from "@/components/ui/button/button.svelte";
    import { getContext } from "svelte";
    import { ytKey } from "./yt-keys";
    import type { YouTubePlayerContext } from "./types";

    let { seekTime } = $props();

    const { getPlayer, getReady } = getContext<YouTubePlayerContext>(ytKey);

    let player = $derived(getPlayer());
    let isReady = $derived(getReady());
    let seekTimeRelased = $state(false);

    function seekTo(allowSeekAhead: boolean = false) {
        player?.seekTo(seekTime, allowSeekAhead);
    }

    function playeVideo() {
        player?.playVideo();
    }
</script>

<div>
    <Button onclick={playeVideo} disabled={!isReady}>PlayeVideo</Button>
    <Button onclick={() => seekTo(false)} disabled={!isReady}>SeekTo</Button>
    <Button onclick={() => seekTo(true)} disabled={!isReady}>SeetPlay</Button>
</div>
