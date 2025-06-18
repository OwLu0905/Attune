<script lang="ts">
    import RecordHistoryItem from "./record-history-item.svelte";
    import type { RecordHistoryData } from "./record-history-data.svelte";

    interface Props {
        audioId: string;
        questionId: string;
        recordData: RecordHistoryData;
    }
    let { audioId, questionId, recordData }: Props = $props();
</script>

{#await recordData.getData(audioId, questionId)}
    <p>...loading</p>
{:then recordFiles}
    <div class="flex flex-wrap gap-2">
        {#each recordFiles as file (file.name)}
            <RecordHistoryItem
                {audioId}
                {questionId}
                data={file.data}
                name={file.name}
                {recordData}
            />
        {/each}
    </div>
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}
