<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import * as HoverCard from "@/components/ui/hover-card/";
    import type { AudioListItem } from "@/tauri";
    import { cn } from "@/utils";
    import { Calendar } from "@lucide/svelte";
    import { fade } from "svelte/transition";

    interface Props {
        echoId: string;
        audioList: AudioListItem[];
    }

    type GroupAudioList = {
        yearMonth: number;
        data: AudioListItem[];
    }[];

    let { echoId, audioList }: Props = $props();

    let groupedAudioList: GroupAudioList = $derived.by(() => {
        // Group audioList by year-month
        const grouped = new Map<number, AudioListItem[]>();

        audioList.forEach((audio) => {
            const date = new Date(audio.updatedAt);
            const yearMonth = date.getFullYear() * 100 + (date.getMonth() + 1); // e.g., 202412 for Dec 2024

            if (!grouped.has(yearMonth)) {
                grouped.set(yearMonth, []);
            }
            grouped.get(yearMonth)!.push(audio);
        });

        // Convert to array and sort by yearMonth descending (most recent first)
        return Array.from(grouped.entries())
            .map(([yearMonth, data]) => ({
                yearMonth,
                data: data.sort(
                    (a, b) =>
                        new Date(b.updatedAt).getTime() -
                        new Date(a.updatedAt).getTime(),
                ),
            }))
            .sort((a, b) => b.yearMonth - a.yearMonth);
    });
</script>

{#each groupedAudioList as group (group.yearMonth)}
    <Sidebar.GroupLabel class="gap-1.5">
        <Calendar />
        <time>
            {group.yearMonth}
        </time>
    </Sidebar.GroupLabel>

    <Sidebar.Menu>
        {#each group.data as audio (audio.id)}
            <Sidebar.MenuItem
                class="min-w-0 group-data-[collapsible=icon]:hidden"
            >
                <HoverCard.Root>
                    <HoverCard.Trigger>
                        <Sidebar.MenuButton
                            class={cn(
                                "",
                                echoId === audio.id && "text-primary",
                            )}
                        >
                            {#snippet child({ props })}
                                <a
                                    href="/echo/{audio.id}"
                                    class="truncate font-mono"
                                    {...props}
                                >
                                    <span out:fade>
                                        {audio.title}
                                    </span>
                                </a>
                            {/snippet}
                        </Sidebar.MenuButton>
                    </HoverCard.Trigger>
                    <HoverCard.Content side="right">
                        <img
                            class="aspect-video h-20 object-cover"
                            src={audio.thumbnail}
                            alt={audio.title}
                        />
                    </HoverCard.Content>
                </HoverCard.Root>
            </Sidebar.MenuItem>
        {/each}
    </Sidebar.Menu>
{/each}
