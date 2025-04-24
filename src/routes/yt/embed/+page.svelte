<script lang="ts">
    import { PlayOrYt, ActionOrYt } from "@/components/youtue";

    let videos = $state([
        {
            id: "dQw4w9WgXcQ",
            title: "Rick Astley - Never Gonna Give You Up",
            seekTime: 20,
        },
        {
            id: "ZTFTngOG2bg",
            title: "Introduction to Svelte",
            seekTime: 40,
        },
    ]);
</script>

{#each videos as video (video.id)}
    <PlayOrYt videoId={video.id}>
        {#snippet action()}
            <ActionOrYt seekTime={video.seekTime} />
        {/snippet}
    </PlayOrYt>
{/each}

<!-- 
{#each videos as video (video.id)}
    <PlayOrYt videoId={video.id} />
{/each} 
-->
