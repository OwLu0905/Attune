<script lang="ts">
    import { onMount } from "svelte";
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
        Pause,
        Play,
        RotateCcw,
    } from "@lucide/svelte";

    import type { AudioPlayer } from "./audio-player.svelte";
    import type { SubtitleSegment } from "./types";
    import { PLAYBACK_RATE } from "@/constants";

    interface Props {
        questionId: string;
        audioPlayer: AudioPlayer;
        onPause: () => Promise<void>;
        onPlaySection: (start: number, end: number) => Promise<void>;
        subtitles: SubtitleSegment[];
    }
    let { questionId, audioPlayer, onPause, onPlaySection, subtitles }: Props =
        $props();

    let hiddenAll = $state(true);
    let dictationList = $state<{ dictationId: number; createdAt: string }[]>(
        [],
    );

    let hiddenItem = $derived.by(() => {
        questionId;
        return hiddenAll;
    });

    let playrate = $state("1");

    let audioId = $derived(page.params.id);

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
        // TODO: get dictation list
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

<div class="flex flex-col gap-1 px-4 py-2 tabular-nums">
    <div class="flex justify-between gap-4">
        <Button
            variant="outline"
            size="sm"
            onclick={() => {
                if (+questionId === 0) return;
                questionId = `${+questionId - 1}`;
            }}
        >
            <ChevronLeft />
        </Button>
        <div class="flex items-center gap-4 text-sm">
            <Button onclick={() => (hiddenAll = !hiddenAll)} variant="outline">
                {#if hiddenAll}
                    <Eye class="text-primary" />
                {:else}
                    <EyeOff class="text-primary" />
                {/if}
            </Button>
            <Select.Root type="single" bind:value={questionId}>
                <Select.Trigger class="w-20">{+questionId + 1}</Select.Trigger>
                <Select.Content>
                    {#each items as s}
                        <Select.Item value={s}>{+s + 1}</Select.Item>
                    {/each}
                </Select.Content>
            </Select.Root>
        </div>
        <Button
            variant="outline"
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

    <div class="shrink grow">
        {#key questionId}
            <section class=" bg-card flex flex-col gap-2 p-4">
                <div class="flex flex-wrap gap-1 p-1 tracking-wide" in:fade>
                    <SegmentField
                        {audioPlayer}
                        segment={subtitles?.[+questionId]}
                        hidden={hiddenItem}
                    />
                </div>
                <div class="w-full">
                    <Textarea />
                </div>

                <div class="flex items-center justify-center gap-2">
                    <ToggleGroup.Root
                        size="sm"
                        type="single"
                        bind:value={
                            () => playrate,
                            (v) => {
                                playrate = v;
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
                        variant="link"
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
                            <Pause class="text-primary" />
                        {:else}
                            <Play class="text-primary" />
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
                </div>
            </section>
        {/key}
    </div>
</div>
