<script lang="ts">
    import PlayOrYt from "@/components/youtue/play-or-yt.svelte";

    let videos = $state([
        {
            id: "dQw4w9WgXcQ",
            title: "Rick Astley - Never Gonna Give You Up",
        },
        {
            id: "ZTFTngOG2bg",
            title: "Introduction to Svelte",
        },
    ]);
</script>

{#each videos as video (video.id)}
    <PlayOrYt videoId={video.id} />
{/each}
