<script lang="ts">
    import { cn } from "@/utils";
    import SegmentField from "./segment-field.svelte";

    import type { SubtitleSegment } from "./types";
    import type { AudioPlayer } from "./audio-player.svelte";
    import type { BookmarkDictationView } from "$lib/tauri";

    interface Props {
        audioPlayer: AudioPlayer;
        subtitles: SubtitleSegment[];
        hidden: boolean;
        onPause: () => Promise<void>;
        onPlaySection: (
            start: number,
            end: number,
            setEnd?: boolean,
        ) => Promise<void>;
        dictationId: number;
        getDictation: (i: number, st: number) => void;
        combinedList: BookmarkDictationView[];
        createBookmarkItem: (i: number) => Promise<void>;
        deleteBookmarkItem: (i: number) => Promise<void>;
        searchText: string;
    }
    let {
        audioPlayer,
        subtitles: subtitlesRaw,
        hidden,
        onPause,
        onPlaySection,
        dictationId,
        getDictation,
        combinedList,
        createBookmarkItem,
        deleteBookmarkItem,
        searchText,
    }: Props = $props();

    let subtitles = $derived.by(() => {
        if (searchText === "") {
            return subtitlesRaw;
        }
        return subtitlesRaw.filter((i) => {
            return i.text.toLowerCase().includes(searchText);
        });
    });
</script>

<div
    class={cn(
        "transition-colors duration-300 ease-linear",
        searchText && "from-primary/20 rounded-xl bg-gradient-to-br",
    )}
>
    {#each subtitles as segment, index (index)}
        <SegmentField
            {audioPlayer}
            {segment}
            {hidden}
            {onPause}
            {onPlaySection}
            {index}
            {dictationId}
            {getDictation}
            {combinedList}
            {createBookmarkItem}
            {deleteBookmarkItem}
        />
    {/each}
</div>
