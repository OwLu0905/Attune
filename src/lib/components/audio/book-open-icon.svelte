<script lang="ts">
    import { cn } from "@/utils";
    import { Check, MousePointerClick, SquarePen } from "@lucide/svelte";

    interface Props {
        isDictation: boolean;
        isActive: boolean;
    }

    let { isDictation, isActive }: Props = $props();
</script>

{#snippet active(isDictation: boolean, isActive: boolean)}
    {#if isDictation}
        <Check class="stroke-foreground h-6 w-4" />
    {:else}
        <SquarePen
            class={cn(
                "stroke-primary-foreground/10 h-6 w-4 cursor-pointer stroke-2",
                isActive && "stroke-primary-foreground stroke-1",
            )}
        />
    {/if}
{/snippet}

<div
    class={cn("book-container h-6 w-5", !isActive && "pulse")}
    aria-label="book"
>
    {@render active(isDictation, isActive)}
</div>
