<!--
 * @author: @kokonutui
 * @description: AI Voice
 * @version: 1.0.0 (converted to Svelte 5)
 * @date: 2025-06-26
 * @license: MIT
 * @website: https://kokonutui.com
 * @github: https://github.com/kokonut-labs/kokonutui
-->

<script lang="ts">
    import { cn } from "@/utils";
    import { onMount, onDestroy } from "svelte";

    let submitted = $state(false);
    let time = $state(0);
    let isClient = $state(false);
    let isDemo = $state(true);

    let timeInterval: number | undefined = undefined;
    let demoTimeout: number | undefined = undefined;
    let animationTimeout: number | undefined = undefined;

    onMount(() => {
        isClient = true;

        // Demo animation effect
        if (isDemo) {
            const runAnimation = () => {
                submitted = true;
                animationTimeout = setTimeout(() => {
                    submitted = false;
                    animationTimeout = setTimeout(runAnimation, 1000);
                }, 3000);
            };

            demoTimeout = setTimeout(runAnimation, 100);
        }
    });

    onDestroy(() => {
        if (timeInterval) clearInterval(timeInterval);

        if (demoTimeout) clearTimeout(demoTimeout);

        if (animationTimeout) clearTimeout(animationTimeout);
    });

    // Timer effect
    $effect(() => {
        if (timeInterval) clearInterval(timeInterval);

        if (submitted) {
            timeInterval = setInterval(() => {
                time += 1;
            }, 1000);
        } else {
            time = 0;
        }
    });

    // Demo cleanup effect
    $effect(() => {
        if (!isDemo) {
            if (demoTimeout) clearTimeout(demoTimeout);
            if (animationTimeout) clearTimeout(animationTimeout);
        }
    });

    // @ts-nocheck
    function formatTime(seconds: any) {
        const mins = Math.floor(seconds / 60);
        const secs = seconds % 60;
        return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
    }

    function handleClick() {
        if (isDemo) {
            isDemo = false;
            submitted = false;
        } else {
            submitted = !submitted;
        }
    }
</script>

<div class="w-full py-4">
    <div
        class="relative mx-auto flex w-full max-w-xl flex-col items-center gap-2"
    >
        <button
            class={cn(
                "group flex h-16 w-16 items-center justify-center rounded-xl transition-colors",
                submitted
                    ? "bg-none"
                    : "bg-none hover:bg-black/5 dark:hover:bg-white/5",
            )}
            type="button"
            onclick={handleClick}
        >
            {#if submitted}
                <div
                    class="pointer-events-auto h-6 w-6 animate-spin cursor-pointer rounded-sm bg-black dark:bg-white"
                    style="animation-duration: 3s;"
                ></div>
            {:else}
                <!-- Mic icon (using a simple SVG since lucide-react isn't available) -->
                <svg
                    class="h-6 w-6 text-black/90 dark:text-white/90"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
                    />
                </svg>
            {/if}
        </button>

        <span
            class={cn(
                "font-mono text-sm transition-opacity duration-300",
                submitted
                    ? "text-black/70 dark:text-white/70"
                    : "text-black/30 dark:text-white/30",
            )}
        >
            {formatTime(time)}
        </span>

        <div class="flex h-4 w-64 items-center justify-center gap-0.5">
            {#each Array(48) as _, i}
                <div
                    class={cn(
                        "w-0.5 rounded-full transition-all duration-300",
                        submitted
                            ? "animate-pulse bg-black/50 dark:bg-white/50"
                            : "h-1 bg-black/10 dark:bg-white/10",
                    )}
                    style={submitted && isClient
                        ? `height: ${20 + Math.random() * 80}%; animation-delay: ${i * 0.05}s;`
                        : undefined}
                ></div>
            {/each}
        </div>

        <p class="h-4 text-xs text-black/70 dark:text-white/70">
            {submitted ? "Listening..." : "Click to speak"}
        </p>
    </div>
</div>

<style>
    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }

    .animate-spin {
        animation: spin 1s linear infinite;
    }

    @keyframes pulse {
        0%,
        100% {
            opacity: 1;
        }
        50% {
            opacity: 0.5;
        }
    }

    .animate-pulse {
        animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
    }
</style>
