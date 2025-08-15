<script lang="ts">
    import { PLAYBACK_BUFFER } from "@/utils";
    import Badge from "../ui/badge/badge.svelte";
    import { fade } from "svelte/transition";

    import type { SubtitleSegment } from "../audio/types";
    import type { AudioPlayer } from "../audio/audio-player.svelte";
    import { type BookmarkDictationView } from "$lib/tauri";
    import * as Tabs from "$lib/components/ui/tabs";
    import RecordRegion from "../audio/record/record-region.svelte";
    import { RecordHistoryData } from "../audio/record/record-history-data.svelte";
    import RecordHistoryCard from "../audio/record/record-history-card.svelte";

    import DictationTextEditor from "./dictation-text-editor.svelte";

    interface Props {
        audioId: string;
        dictationId: number;
        combinedList: BookmarkDictationView[];
        length: number;
        dictationItem: SubtitleSegment;
        audioPlayer: AudioPlayer;
        onPause: () => Promise<void>;
        onPlaySection: (start: number, end: number) => Promise<void>;
    }

    let {
        audioId,
        dictationId = $bindable(),
        combinedList = $bindable(),
        length,
        dictationItem,
        audioPlayer,
        onPause,
        onPlaySection,
    }: Props = $props();

    const recordData = new RecordHistoryData();

    let activeTab = $state("dictation");

    function onNext() {
        onPause();
        if (dictationId === length - 1) {
            dictationId = 0;
        } else {
            dictationId++;
        }
    }

    function onPrev() {
        onPause();
        if (dictationId === 0) {
            dictationId = length - 1;
        } else {
            dictationId--;
        }
    }
</script>

<div class="mb-2 flex items-center justify-end">
    <Badge variant="secondary" class="text-primary tabular-nums">
        <span in:fade>{dictationId} </span>/ {length - 1}
    </Badge>
</div>

<Tabs.Root bind:value={activeTab}>
    <Tabs.List>
        <Tabs.Trigger value="dictation">Dictation</Tabs.Trigger>
        <Tabs.Trigger value="echoing">Echoing</Tabs.Trigger>
    </Tabs.List>

    {#key dictationId}
        <Tabs.Content value="dictation">
            <DictationTextEditor
                {audioId}
                {dictationId}
                bind:combinedList
                {dictationItem}
                {audioPlayer}
                {onPause}
                {onPlaySection}
                {onPrev}
                {onNext}
            />
        </Tabs.Content>
    {/key}

    {#key dictationId}
        <Tabs.Content value="echoing">
            <div class="text-muted-foreground p-4 text-center">
                <RecordRegion
                    {audioId}
                    {dictationId}
                    {recordData}
                    {onPrev}
                    {onNext}
                />
            </div>
            <section class="my-2 flex shrink grow flex-col gap-2">
                <RecordHistoryCard {audioId} {dictationId} {recordData} />
            </section>
        </Tabs.Content>
    {/key}
</Tabs.Root>
