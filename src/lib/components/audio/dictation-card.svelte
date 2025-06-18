<script lang="ts">
    import { onMount, type Component } from "svelte";
    import { fade } from "svelte/transition";
    import { invoke } from "@tauri-apps/api/core";
    import { page } from "$app/state";
    import { getUserContext } from "@/user/userService.svelte";

    import Button from "$lib/components/ui/button/button.svelte";
    import Textarea from "@/components/ui/textarea/textarea.svelte";
    import * as Select from "@/components/ui/select";
    import * as ToggleGroup from "@/components/ui/toggle-group";

    import SegmentField from "./segment-field.svelte";

    import {
        ChevronLeft,
        ChevronRight,
        Eye,
        EyeOff,
        Heart,
        Mic,
        Pause,
        Play,
        RotateCcw,
        SquareArrowOutUpRight,
        SquarePen,
        Trash,
    } from "@lucide/svelte";

    import type { AudioPlayer } from "./audio-player.svelte";
    import type { SubtitleSegment } from "./types";
    import { PLAYBACK_RATE } from "@/constants";
    import { cn } from "@/utils";
    import RecordRegion from "./record/record-region.svelte";
    import RecordHistoryCard from "./record/record-history-card.svelte";
    import { RecordHistoryData } from "./record/record-history-data.svelte";

    interface Props {
        audioId: string;
        questionId: string;
        audioPlayer: AudioPlayer;
        onPause: () => Promise<void>;
        onPlaySection: (start: number, end: number) => Promise<void>;
        subtitles: SubtitleSegment[];
        hiddenAll: boolean;
    }
    let {
        questionId,
        audioPlayer,
        onPause,
        onPlaySection,
        subtitles,
        hiddenAll,
    }: Props = $props();

    let dictationList = $state<{ dictationId: number; createdAt: string }[]>(
        [],
    );

    let hiddenItem = $derived.by(() => {
        questionId;
        return hiddenAll;
    });

    let playbackRate = $state("1");
    let selected = $state("mic");

    let audioId = $derived(page.params.id);

    const recordData = new RecordHistoryData();

    const { getUser } = getUserContext();

    const user = getUser();

    const items = $derived(
        Array.from({ length: subtitles.length }, (_, i) => `${i}`),
    );

    const isLove = $derived.by(
        () =>
            dictationList.findIndex((i) => i.dictationId === +questionId) > -1,
    );

    onMount(() => {
        async function getDictationList() {
            try {
                dictationList = await invoke("handle_get_dictation_list", {
                    token: user.accessToken,
                    audio_id: audioId,
                });
            } catch (error) {
                console.error(error);
            }
        }
        getDictationList();
    });

    $effect(() => {
        questionId;
        return () => {
            audioPlayer.onPause();
        };
    });

    $effect(() => {
        if (
            audioPlayer?.currentTime > subtitles?.[+questionId].end &&
            audioPlayer.isPlaying
        ) {
            audioPlayer.onPause();
        }
    });
</script>

{#snippet item(value: string, Icon: Component)}
    <button
        class={cn(
            "bg-card text-primary flex items-center justify-center rounded-full shadow-md",
            "origin-bottom-right transition-all duration-300 ease-[cubic-bezier(0.4,0,0.2,1)]",
            "h-0 w-0 scale-0 opacity-0",
            "group-hover:h-10 data-[current=true]:h-10",
            "group-hover:w-10 data-[current=true]:w-10",
            "group-hover:scale-100 data-[current=true]:scale-100",
            "group-hover:opacity-100 data-[current=true]:opacity-100",
            "group-hover:data-[current=true]:translate-x-0.5 ",
            "nth-1:-translate-y-4",
            "nth-2:-translate-y-2",
            "data-[current=true]:inset-shadow-sm",
            "data-[current=true]:inset-shadow-primary/60 ",
        )}
        data-current={selected === value ? "true" : "false"}
        onclick={() => {
            selected = value;
        }}
    >
        <Icon class="max-h-4 max-w-4" />
    </button>
{/snippet}

<div class="flex flex-col gap-1 overflow-auto px-4 py-2 tabular-nums">
    <div class="flex shrink grow flex-col gap-2 overflow-auto">
        {#key questionId}
            <section class="bg-card flex flex-col gap-2 p-4">
                <div
                    class="flex shrink-0 flex-wrap gap-1 p-1 tracking-wide"
                    in:fade
                >
                    <SegmentField
                        {audioPlayer}
                        segment={subtitles?.[+questionId]}
                        hidden={hiddenItem}
                    />
                </div>
                <div class="flex items-stretch">
                    {#if selected === "pen"}
                        <Textarea class="min-h-16" />
                    {:else if selected === "mic"}
                        <RecordRegion {audioId} {questionId} {recordData} />
                    {/if}
                </div>
            </section>

            <div
                class="group absolute right-4 bottom-4 flex h-auto flex-col px-2 py-1"
            >
                {@render item("link", SquareArrowOutUpRight)}
                {@render item("pen", SquarePen)}
                {@render item("mic", Mic)}
            </div>

            <div
                class="absolute right-0 bottom-4 left-0 mx-auto flex w-fit items-center justify-center gap-2 py-1"
            >
                <Button
                    variant="ghost"
                    size="sm"
                    onclick={() => {
                        if (+questionId === 0) return;
                        questionId = `${+questionId - 1}`;
                    }}
                >
                    <ChevronLeft />
                </Button>
                <div
                    class="bg-card inset-shadow-primary/60 flex items-center justify-center gap-2 rounded-full px-8 py-1 shadow-md inset-shadow-sm"
                >
                    <ToggleGroup.Root
                        size="sm"
                        type="single"
                        bind:value={
                            () => playbackRate,
                            (v) => {
                                playbackRate = v;
                                audioPlayer.onSetPlaybackRate(+v);
                                return v;
                            }
                        }
                    >
                        {#each PLAYBACK_RATE as speed}
                            <ToggleGroup.Item
                                value={`${speed}`}
                                class="text-xs tabular-nums"
                            >
                                {speed.toFixed(2)}
                            </ToggleGroup.Item>
                        {/each}
                    </ToggleGroup.Root>
                    <Button
                        onclick={() => (hiddenItem = !hiddenItem)}
                        variant="link"
                    >
                        {#if hiddenItem}
                            <Eye class="text-primary" />
                        {:else}
                            <EyeOff class="text-primary" />
                        {/if}
                    </Button>

                    <Button
                        variant="secondary"
                        size="icon"
                        class="size-8"
                        onclick={() => {
                            if (!audioPlayer) return;

                            if (audioPlayer.isPlaying) {
                                onPause();
                            } else {
                                const startTime =
                                    subtitles?.[+questionId].start <=
                                        audioPlayer?.currentTime &&
                                    audioPlayer?.currentTime <=
                                        subtitles?.[+questionId].end
                                        ? audioPlayer?.currentTime
                                        : subtitles?.[+questionId].start;

                                onPlaySection(
                                    startTime,
                                    subtitles?.[+questionId].end,
                                );
                            }
                        }}
                    >
                        {#if audioPlayer?.isPlaying}
                            <Pause />
                        {:else}
                            <Play />
                        {/if}
                    </Button>
                    <Button
                        variant="link"
                        onclick={() => {
                            onPlaySection(
                                subtitles?.[+questionId].start,
                                subtitles?.[+questionId].end,
                            );
                        }}
                    >
                        <RotateCcw class="w-5 text-lime-500" />
                    </Button>

                    {#if isLove}
                        <div in:fade>
                            <Button
                                variant="link"
                                onclick={async () => {
                                    dictationList = await invoke(
                                        "handle_delete_dictation_item",
                                        {
                                            token: user.accessToken,
                                            audio_id: audioId,
                                            dictation_id: +questionId,
                                        },
                                    );
                                }}
                            >
                                <Heart class="fill-rose-500 stroke-red-500" />
                            </Button>
                        </div>
                    {:else}
                        <div in:fade>
                            <Button
                                variant="link"
                                onclick={async () => {
                                    dictationList = (await invoke(
                                        "handle_create_dictation_item",
                                        {
                                            token: user.accessToken,
                                            audio_id: audioId,
                                            dictation_id: +questionId,
                                        },
                                    )) as {
                                        dictationId: number;
                                        createdAt: string;
                                    }[];
                                }}
                            >
                                <Heart class="w-5 stroke-red-500" />
                            </Button>
                        </div>
                    {/if}

                    <Select.Root type="single" bind:value={questionId}>
                        <Select.Trigger class="border-primary/10 w-16 text-xs"
                            >{+questionId + 1}</Select.Trigger
                        >
                        <Select.Content>
                            {#each items as s}
                                <Select.Item value={s}>{+s + 1}</Select.Item>
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>

                <Button
                    variant="ghost"
                    size="sm"
                    onclick={() => {
                        if (+questionId === subtitles?.length - 1) {
                            return;
                        }
                        questionId = `${+questionId + 1}`;
                    }}
                >
                    <ChevronRight />
                </Button>
            </div>
            <section class="my-2 flex shrink grow flex-col gap-2">
                <RecordHistoryCard {audioId} {questionId} {recordData} />
            </section>
        {/key}
    </div>
</div>
