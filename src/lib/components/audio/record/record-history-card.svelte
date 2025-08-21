<script lang="ts">
    import RecordHistoryItem from "./record-history-item.svelte";
    import type { RecordHistoryData } from "./record-history-data.svelte";

    interface Props {
        audioId: string;
        dictationId: number;
        recordData: RecordHistoryData;
    }
    let { audioId, dictationId, recordData }: Props = $props();
</script>

{#await recordData.getData(audioId, dictationId) then recordFiles}
    <div class="flex flex-col gap-2 overflow-scroll">
        {#each recordFiles as file (file.name)}
            <RecordHistoryItem
                {audioId}
                {dictationId}
                data={file.data}
                name={file.name}
                {recordData}
            />
        {/each}
    </div>
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}
