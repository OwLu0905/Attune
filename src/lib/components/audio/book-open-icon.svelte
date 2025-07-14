<script lang="ts">
    import { cn } from "@/utils";
    import { Check } from "@lucide/svelte";

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
        <Check class="h-6 w-5 stroke-1 opacity-30" />
    {/if}
{/snippet}

<div
    class={cn("book-container h-6 w-5", !isActive && "pulse")}
    aria-label="book"
>
    {@render active(isDictation, isActive)}
</div>
