<script>
  import WaveAudio from "@/components/wave/WaveAudio.svelte";
  let canvas;
</script>

<WaveAudio bind:this={canvas} />

<button
  onclick={() => {
    canvas.trigger();
  }}>ccc</button
>
