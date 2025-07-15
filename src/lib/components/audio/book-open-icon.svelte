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
        <Check class="h-6 w-4 stroke-emerald-400" />
    {:else}
        <SquarePen
            class={cn(
                "h-6 w-4 cursor-pointer stroke-2 opacity-10",
                isActive && "stroke-primary stroke-2 opacity-100",
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
