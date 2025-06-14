<script lang="ts">
    import { cn } from "@/utils";
    import Button from "@/components/ui/button/button.svelte";
    import * as Select from "@/components/ui/select";
    import { Disc, Mic } from "@lucide/svelte";
    import WaveBackground from "./wave-background.svelte";
    import { onMount } from "svelte";
    import WaveSurfer from "wavesurfer.js";
    import RecordPlugin from "wavesurfer.js/dist/plugins/record.esm.js";

    let rc: HTMLElement;
    let ws;
    let record: any;
    let micSelect: any[] = [];

    onMount(() => {
        ws = WaveSurfer.create({
            container: rc,
            waveColor: "rgb(200, 0, 200)",
            progressColor: "rgb(100, 0, 100)",
        });
        record = ws.registerPlugin(
            RecordPlugin.create({
                renderRecordedAudio: false,
                scrollingWaveform: true,
                continuousWaveform: true,
                continuousWaveformDuration: 30, // optional
            }),
        );
    });

    onMount(() => {
        async function getAudioDevices() {
            try {
                // Request permission first (required for device labels)
                await navigator.mediaDevices.getUserMedia({ audio: true });

                // Get all media devices
                const devices = await navigator.mediaDevices.enumerateDevices();

                // Filter for audio input devices (microphones)
                const audioInputs = devices.filter(
                    (device) => device.kind === "audioinput",
                );

                // Filter for audio output devices (speakers/headphones)
                const audioOutputs = devices.filter(
                    (device) => device.kind === "audiooutput",
                );

                // console.log('Audio Input Devices:', audioInputs);
                // console.log('Audio Output Devices:', audioOutputs);

                return { audioInputs, audioOutputs };
            } catch (error) {
                console.error("Error accessing media devices:", error);
            }
        }

        getAudioDevices().then((data) => {
            console.log(data);
            micSelect = data?.audioInputs ?? [];
        });
    });
</script>

<div
    bind:this={rc}
    class={cn(
        "border-border box-border h-16 w-full rounded-md border shadow-xs",
        "flex items-center justify-center",
        "relative",
        "backdrop-blur-2xl",
    )}
>
    <!-- <WaveBackground /> -->
    <div
        class="z-5 flex h-full w-full items-center justify-center gap-2 bg-white/30 backdrop-blur-3xl"
    >
        <Button
            size="sm"
            onclick={async () => {
                await record.startRecording({ deviceId: "" });
            }}
        >
            <Mic strokeWidth={1.5} />
            Record</Button
        >

        <Button
            size="sm"
            variant="destructive"
            onclick={() => {
                record.stopRecording();
            }}
        >
            <Disc strokeWidth={1.5} />
            Finish</Button
        >
    </div>
</div>

<Select.Root type="single">
    <Select.Trigger class="mx-4 w-[180px]"></Select.Trigger>
    <Select.Content>
        {#each micSelect as mic}
            <Select.Item value={mic.deviceId}>{mic.labels}</Select.Item>
        {/each}
    </Select.Content>
</Select.Root>
