<script module>
    console.log("YouTube API Loading...");

    let YouTube: typeof YT.Player | null = $state(null);
    function tryAssignYouTube() {
        if (!YouTube && window?.YT?.Player) {
            YouTube = window.YT.Player;
        }
    }

    if (!window.onYouTubeIframeAPIReady) {
        window.onYouTubeIframeAPIReady = () => {
            tryAssignYouTube();
        };
    }

    if (!document.querySelector('script[src*="youtube.com/iframe_api"]')) {
        const tag = document.createElement("script");
        tag.src = "https://www.youtube.com/iframe_api";
        const firstScriptTag = document.getElementsByTagName("script")[0];
        firstScriptTag?.parentNode?.insertBefore(tag, firstScriptTag);
    } else {
        tryAssignYouTube();
    }
</script>

<script lang="ts">
    import { getContext, onDestroy, setContext, type Snippet } from "svelte";

    import { ytKey } from "./yt-keys";
    import { sliderValuesKey } from "./yt-keys";

    import type { YouTubePlayerContext, YouTubeSliderContext } from "./types";

    interface Props {
        action?: Snippet;
        videoId: string;
    }

    let playerContainer: HTMLElement | null = $state(null);

    let isReady = $state(false);
    let initError = $state<string | null>(null);

    let player: YT.Player | null = $state(null);

    let { videoId, action }: Props = $props();

    let thumbnailUrl = $derived(
        `https://i.ytimg.com/vi/${videoId}/hqdefault.jpg`,
    );

    let liteLoaded = $state(false);

    setContext<YouTubePlayerContext>(ytKey, {
        getPlayer: () => player,
        getReady: () => isReady,
        getError: () => initError,
    });

    const { getSliderValues } =
        getContext<YouTubeSliderContext>(sliderValuesKey);

    const sliderValues = getSliderValues();

    function initialize() {
        if (!YouTube || player) {
            console.warn("YouTube not ready or player already initialized");
            return;
        }

        if (!playerContainer) {
            console.warn("YouTube can't find the container");
            return;
        }

        try {
            // TODO: Performance: lite loading? (ignore this, I'll do it later by myself)
            player = new YouTube(playerContainer, {
                videoId,
                playerVars: {
                    autoplay: !action && liteLoaded ? 1 : 0,
                },
                events: {
                    onReady: (event) => {
                        console.log("Ready", videoId);
                        isReady = true;
                        sliderValues[1] = event.target.getDuration();
                    },
                    onError: (e) => {
                        console.error("YouTube player error:", e);
                        initError = "Failed to play video";
                    },
                },
            });
        } catch (err) {
            console.error("YouTube player initialization error:", err);
            initError = "Failed to initialize player";
        }
    }

    $effect(() => {
        tryAssignYouTube();

        if (action || liteLoaded) {
            initialize();
        }
    });

    onDestroy(() => {
        if (player) {
            player.destroy();
        }
    });
</script>

{#if !action && !liteLoaded}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
        aria-label="Play YouTube video"
        role="button"
        tabindex="0"
        onclick={() => {
            liteLoaded = true;
        }}
        class="relative aspect-video h-[360px] w-[640px] max-w-full cursor-pointer overflow-hidden bg-[#000] bg-cover bg-center bg-repeat"
        style="background-image:url({thumbnailUrl});"
    >
        <div
            class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2"
        >
            <svg viewBox="0 0 68 48" width="68" height="48">
                <path
                    d="M66.52,7.74c-0.78-2.93-2.49-5.41-5.42-6.19C55.79,.13,34,0,34,0S12.21,.13,6.9,1.55 C3.97,2.33,2.27,4.81,1.48,7.74C0.06,13.05,0,24,0,24s0.06,10.95,1.48,16.26c0.78,2.93,2.49,5.41,5.42,6.19 C12.21,47.87,34,48,34,48s21.79-0.13,27.1-1.55c2.93-0.78,4.64-3.26,5.42-6.19C67.94,34.95,68,24,68,24S67.94,13.05,66.52,7.74z"
                    fill="#f00"
                ></path>
                <path d="M 45,24 27,14 27,34" fill="#fff"></path>
            </svg>
        </div>
    </div>
{/if}

{#if action || liteLoaded}
    <div
        class="aspect-video self-center py-2"
        bind:this={playerContainer}
    ></div>
    {#if action && isReady}
        {@render action()}
    {/if}
{/if}
