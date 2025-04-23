<script module>
    let Youtube: typeof YT.Player | null = $state(null);
    window.onYouTubeIframeAPIReady = () => {
        Youtube = window.YT.Player;
    };
    if (!document.querySelector('script[src*="youtube.com/iframe_api"]')) {
        let tag = document.createElement("script");
        tag.src = "https://www.youtube.com/iframe_api";
        let firstScriptTag = document.getElementsByTagName("script")[0];
        firstScriptTag?.parentNode?.insertBefore(tag, firstScriptTag);
    }
</script>

<script lang="ts">
    import { onMount } from "svelte";

    let playerContainer: HTMLElement;

    let isApiLoaded = $state(true);
    let player: any = $state(null);

    let { videoId } = $props();

    function initialize() {
        if (!Youtube) return;

        player = new Youtube(playerContainer, {
            videoId,
            events: {
                onReady: () => {
                    console.log("Ready", videoId);
                },
            },
        });
    }
    onMount(() => {
        if (!Youtube) return;
        initialize();
    });
</script>

<div bind:this={playerContainer}></div>
