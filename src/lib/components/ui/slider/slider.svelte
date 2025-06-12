<script lang="ts">
    import { Slider as SliderPrimitive } from "bits-ui";
    import type { WithoutChildrenOrChild } from "$lib/utils.js";
    import { cn, simpleFormatSecondsToMMSS } from "$lib/utils.js";

    let {
        ref = $bindable(null),
        value = $bindable(),
        orientation = "horizontal",
        class: className,
        ...restProps
    }: WithoutChildrenOrChild<SliderPrimitive.RootProps> = $props();
</script>

<!--
Discriminated Unions + Destructing (required for bindable) do not
get along, so we shut typescript up by casting `value` to `never`.
-->
<SliderPrimitive.Root
    bind:ref
    bind:value={value as never}
    {orientation}
    class={cn(
        "relative mb-6 flex touch-none items-center select-none data-[orientation='horizontal']:w-full data-[orientation='vertical']:h-full data-[orientation='vertical']:min-h-44 data-[orientation='vertical']:w-auto data-[orientation='vertical']:flex-col",
        className,
    )}
    {...restProps}
>
    {#snippet children({ thumbs })}
        <span
            data-orientation={orientation}
            class="bg-primary/20 relative grow overflow-hidden rounded-full data-[orientation='horizontal']:h-1.5 data-[orientation='horizontal']:w-full data-[orientation='vertical']:h-full data-[orientation='vertical']:w-1.5"
        >
            <SliderPrimitive.Range
                class="bg-primary absolute data-[orientation='horizontal']:h-full data-[orientation='vertical']:w-full"
            />
        </span>
        {#each thumbs as thumb (thumb)}
            <SliderPrimitive.Thumb
                index={thumb}
                class="border-primary/50 bg-background focus-visible:ring-ring block size-4 rounded-full border shadow-sm transition-colors focus-visible:ring-1 focus-visible:outline-hidden disabled:pointer-events-none disabled:opacity-50"
            >
                {#if restProps.type === "multiple"}
                    <span
                        class="absolute top-4 -left-1/2 text-xs font-light tabular-nums"
                    >
                        {simpleFormatSecondsToMMSS((value as number[])[thumb])}
                    </span>
                {:else}
                    <span
                        class="absolute top-4 left-1/2 -translate-x-1/2 text-xs font-light tabular-nums"
                    >
                        {value}
                    </span>
                {/if}
            </SliderPrimitive.Thumb>
        {/each}
    {/snippet}
</SliderPrimitive.Root>
