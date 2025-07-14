<script lang="ts">
    import { cn } from "@/utils";
    import { Check } from "@lucide/svelte";
    import { fade } from "svelte/transition";

    interface Props {
        isDictation: boolean;
        isActive: boolean;
    }

    let { isDictation, isActive }: Props = $props();
</script>

{#snippet active(isDictation: boolean, isActive: boolean)}
    {#if isDictation}
        <Check class="h-6 w-5 stroke-emerald-400" />
    {:else}
        <Check class="h-6 w-5 stroke-emerald-400/30" />
    {/if}
{/snippet}

<div
    class={cn("book-container h-6 w-5", !isActive && "pulse")}
    aria-label="book"
>
    {#if isActive}
        <div in:fade>
            {@render active(isDictation, isActive)}
        </div>
    {:else}
        {@render active(isDictation, isActive)}
    {/if}
</div>

<style>
    .book-icon {
        width: 20px;
        height: 24px;
        fill: none;
        stroke-width: 1.5px;
        stroke-linecap: round;
        stroke-linejoin: round;
        transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    }

    /* Closed book paths */
    .book-closed {
        opacity: 1;
        transform: scale(1);
    }

    .book-closed.hide {
        opacity: 0;
        transform: scale(0.8) rotate(-10deg);
    }

    /* Open book paths */
    .book-open {
        opacity: 0;
        transform: scale(0.8) rotate(10deg);
    }

    .book-open.show {
        opacity: 1;
        transform: scale(1) rotate(0deg);
    }

    /* Individual path animations */
    .book-open .spine {
        stroke-dasharray: 20;
        stroke-dashoffset: 20;
        transition: stroke-dashoffset 0.6s ease 0.2s;
    }

    .book-open.show .spine {
        stroke-dashoffset: 0;
    }

    .book-open .pages {
        stroke-dasharray: 60;
        stroke-dashoffset: 60;
        transition: stroke-dashoffset 0.8s ease 0.1s;
    }

    .book-open .delay {
        stroke-dasharray: 60;
        stroke-dashoffset: 60;
        transition: stroke-dashoffset 0.8s ease 0.5s;
    }

    .book-open.show .delay {
        stroke-dashoffset: 0;
    }
    .book-open.show .pages {
        stroke-dashoffset: 0;
    }

    .pulse {
        animation: pulse 2s infinite;
    }

    @keyframes pulse {
        0% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.02);
        }
        100% {
            transform: scale(1);
        }
    }
</style>
