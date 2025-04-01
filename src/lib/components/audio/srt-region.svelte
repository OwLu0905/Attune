<script lang="ts">
    import { onMount } from "svelte";
    import { SRT_DATA } from "../../../data/demo-srt";

    export type SubtitleEntity = {
        index: number;
        startTime: number;
        endTime: number;
        text: string;
        highlighted: string | null;
        rawText: string;
    };

    let subtitlesByText: {
        text: string;
        entries: SubtitleEntity[];
        startTime: number;
        endTime: number;
    }[] = $state([]);

    interface Props {
        currentTime: number;
        onClickText: (e: SubtitleEntity) => void;
    }

    let { currentTime, onClickText }: Props = $props();

    function srtTimeToMilliseconds(srtTime: string) {
        // 將時間碼解析為小時、分鐘、秒和毫秒
        const [time, millisec] = srtTime.split(",");
        const [hours, minutes, seconds] = time.split(":").map(Number);

        // 計算總毫秒數
        return (
            hours * 3600000 + // 小時轉毫秒
            minutes * 60000 + // 分鐘轉毫秒
            seconds * 1000 + // 秒轉毫秒
            parseInt(millisec, 10) // 已經是毫秒
        );
    }
    function parseSRT(srtContent: string) {
        const entries: SubtitleEntity[] = [];
        const blocks = srtContent.trim().split(/\n\s*\n/);

        for (const block of blocks) {
            const lines = block.split("\n") as [
                index: string,
                timeCode: string,
                text: string,
            ];

            if (lines.length < 3) continue;

            const index = parseInt(lines[0], 10);
            const timecodeMatch = lines[1].match(
                /(\d{2}:\d{2}:\d{2},\d{3}) --> (\d{2}:\d{2}:\d{2},\d{3})/,
            );

            if (!timecodeMatch) continue;

            const startTimeMilliseconds = srtTimeToMilliseconds(
                timecodeMatch[1],
            );
            const endTimeMilliseconds = srtTimeToMilliseconds(timecodeMatch[2]);

            const rawText = lines.slice(2).join("\n");

            // Extract the highlighted word/phrase
            const highlightMatch = rawText.match(/<u>(.*?)<\/u>/);
            const highlighted = highlightMatch ? highlightMatch[1] : null;

            const text = rawText.replace(/<\/?u>/g, "").trim();

            entries.push({
                index,
                startTime: startTimeMilliseconds,
                endTime: endTimeMilliseconds,
                text,
                highlighted,
                rawText,
            });
        }

        return entries;
    }
    function groupSubtitlesByText(subtitles: SubtitleEntity[]) {
        const textGroups = new Map<string, SubtitleEntity[]>();

        subtitles.forEach((entry) => {
            if (!textGroups.has(entry.text)) {
                textGroups.set(entry.text, []);
            }
            textGroups.get(entry.text)?.push(entry);
        });

        return Array.from(textGroups.entries()).map(([text, entries]) => ({
            text,
            entries: entries.sort((a, b) => a.startTime - b.startTime),
            startTime: entries[0].startTime,
            endTime: entries.at(-1)!.endTime,
        }));
    }

    onMount(async () => {
        const data = parseSRT(SRT_DATA);

        subtitlesByText = groupSubtitlesByText(data);
    });
</script>

<div
    class="text-md flex w-full flex-row flex-wrap gap-0.5 bg-stone-100 px-4 py-2 tabular-nums"
>
    {#each subtitlesByText as i (i.text)}
        <div
            class={`tansition-all flex flex-row flex-wrap rounded-lg duration-100  ease-in-out ${i.endTime > currentTime && currentTime >= i.startTime ? "bg-emerald-200" : ""}`}
        >
            {#each i.entries as entry (entry.index)}
                <button
                    onclick={() => {
                        onClickText(entry);
                    }}
                    class={`tansition-all px-1.5 duration-300 ease-in-out hover:underline ${entry.endTime > currentTime && currentTime >= entry.startTime ? "rounded-2xl bg-violet-300 ring ring-violet-400" : ""} inline tracking-wide last:after:text-red-500 last:after:content-['.']`}
                >
                    {entry.highlighted}
                </button>
            {/each}
        </div>
    {/each}
</div>
