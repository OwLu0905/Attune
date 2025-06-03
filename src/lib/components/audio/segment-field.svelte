<script lang="ts">
    import { cn } from "@/utils";
    import { fade } from "svelte/transition";

    let { audioPlayer, segment, hidden } = $props();

    let currentTime = $derived(audioPlayer.currentTime);
</script>

{#each segment?.words as seg}
    <span
        in:fade
        class={cn(
            "rounded-sm text-primary ",
            "transition-all duration-150 ease-linear",
            hidden ? "bg-primary opacity-10" : "",
            !hidden &&
                seg.end >= currentTime &&
                currentTime >= seg.start &&
                "bg-primary/30",

            hidden &&
                seg.end >= currentTime &&
                currentTime >= seg.start &&
                "opacity-50",
        )}
    >
        {seg.word}
    </span>
{/each}
