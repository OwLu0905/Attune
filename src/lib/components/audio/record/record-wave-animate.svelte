<script lang="ts">
    import { cn } from "@/utils";

    interface Props {
        isLoading: boolean;
    }
    let { isLoading }: Props = $props();
</script>

<div class="flex h-4 w-36 items-center justify-center gap-0.5">
    {#each Array(32) as _, i}
        <div
            class={cn(
                "w-0.5 rounded-full transition-all duration-300",
                isLoading
                    ? "bg-foreground/50 animate-pulse"
                    : "bg-foreground/10 h-2",
            )}
            style={isLoading
                ? `height: ${20 + Math.random() * 80}%; animation-delay: ${i * 0.05}s;`
                : undefined}
        ></div>
    {/each}
</div>

<style>
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
