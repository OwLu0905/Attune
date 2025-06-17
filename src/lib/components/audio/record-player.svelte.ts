import WaveSurfer from "wavesurfer.js";
import RegionsPlugin from "wavesurfer.js/dist/plugins/regions.esm.js";
import RecordPlugin from "wavesurfer.js/dist/plugins/record.esm.js";

import type { Region } from "wavesurfer.js/dist/plugins/regions.esm.js";
import { WAVESURFER_BACKEND } from "@/constants";

export class RecordPlayer {
    ws: WaveSurfer | undefined = $state.raw(undefined);
    activeRegion: Region | null = $state(null);
    regions: RegionsPlugin = RegionsPlugin.create();
    record: RecordPlugin = RecordPlugin.create({
        renderRecordedAudio: false,
        scrollingWaveform: false,
        continuousWaveform: true,
        continuousWaveformDuration: 30, // optional
    });

    currentTime = $state(0);

    isPlaying = $state(false);
    isReady = $state(false);
    isRecording = $state(false);

    computedStyle = getComputedStyle(document.documentElement);
    primaryOklch = this.computedStyle.getPropertyValue("--primary").trim();
    secondaryOklch = this.computedStyle.getPropertyValue("--secondary").trim();

    constructor() {}
    async createRecord(container: HTMLElement) {
        if (this.ws) {
            this.ws.destroy();
        }

        this.ws = WaveSurfer.create({
            container,
            progressColor: `${this.primaryOklch}`,
            waveColor: `${this.secondaryOklch}`,
            barWidth: 2,
            barGap: 1,
            height: 62,
            backend: WAVESURFER_BACKEND,
        });

        this.record = this.ws.registerPlugin(
            RecordPlugin.create({
                renderRecordedAudio: false,
                scrollingWaveform: false,
                continuousWaveform: true,
                continuousWaveformDuration: 30, // optional
            }),
        );

        this.record.on("record-end", (blob) => {
            this.isRecording = false;
            if (this.ws) {
                this.ws.destroy();
            }

            const recordedUrl = URL.createObjectURL(blob);
            this.ws = WaveSurfer.create({
                container,
                progressColor: `${this.primaryOklch}`,
                waveColor: `${this.secondaryOklch}`,
                barWidth: 2,
                barGap: 1,
                height: 62,
                backend: WAVESURFER_BACKEND,
                url: recordedUrl,
            });

            this.ws?.on("click", () => {
                this.ws?.play();
            });
        });
    }
    async onRecord(deviceId: ConstrainDOMString) {
        this.isRecording = true;
        await this.record.startRecording({ deviceId });
    }

    onFinish() {
        this.record.stopRecording();
    }

    async onPlayPause() {
        this.ws!.playPause();
    }
    async onPlay() {
        this.ws!.play();
    }
    async onPause() {
        this.ws!.pause();
    }
    onStop() {
        this.ws!.stop();
    }
    async onPlaySection(start: number, end: number) {
        this.ws!.play(start, end);
    }
    getVolume() {
        return this.ws!.getVolume();
    }
    onSetVolume(sv: number) {
        this.ws!.setVolume(sv);
    }
    onSetPlaybackRate(speed: number) {
        this.ws!.setPlaybackRate(speed, true);
    }
    destory() {
        this.ws?.destroy();
    }
}
