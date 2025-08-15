<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import {
        ChevronLeft,
        ChevronRight,
        Eye,
        Pause,
        Play,
        RotateCcw,
        Save,
        SquareCheckBig,
    } from "@lucide/svelte";

    interface Props {
        variant: "prev" | "next";
        onclick: () => void;
    }

    let { variant, onclick }: Props = $props();

    const iconItems = {
        prev: ChevronLeft,
        next: ChevronRight,
    };

    const Icon = iconItems[variant];
</script>

<Button
    size="sm"
    variant="ghost"
    onclick={() => {
        onclick();
    }}
>
    <Icon />
</Button>
