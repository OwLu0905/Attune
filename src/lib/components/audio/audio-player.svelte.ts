import WaveSurfer from "wavesurfer.js";
import RegionsPlugin from "wavesurfer.js/dist/plugins/regions.esm.js";
import TimelinePlugin from "wavesurfer.js/dist/plugins/timeline.esm.js";
import type { Region } from "wavesurfer.js/dist/plugins/regions.esm.js";

const random = (min: number, max: number) => Math.random() * (max - min) + min;
const randomColor = () =>
    `rgba(${random(0, 255)}, ${random(0, 255)}, ${random(0, 255)}, 0.5)`;

export class AudioPlayer {
    ws: WaveSurfer | undefined = $state.raw(undefined);
    activeRegion: Region | null = $state(null);
    regions: RegionsPlugin = RegionsPlugin.create();
    currentTime = $state(0);

    isPlaying = $state(false);
    isReady = $state(false);
    isTranscribing = $state(false);

    computedStyle = getComputedStyle(document.documentElement);
    primaryOklch = this.computedStyle.getPropertyValue("--primary").trim();
    secondaryOklch = this.computedStyle.getPropertyValue("--secondary").trim();

    constructor(container: HTMLElement) {
        this.ws = WaveSurfer.create({
            container,
            progressColor: `${this.primaryOklch}`,
            waveColor: `${this.secondaryOklch}`,
            barWidth: 2,
            barGap: 1,
            height: 40,
            // backend: WAVESURFER_BACKEND,
            plugins: [this.regions, TimelinePlugin.create()],
        });
    }
    async initialize(audio: BlobPart) {
        if (!this.ws) throw new Error("Can't initialize WaveSurfer");

        this.ws.setVolume(0.5);

        this.ws.on("ready", () => {
            if (!this.ws) throw new Error("Can't initialize WaveSurfer");
            this.isReady = true;
            const ws = this.ws;

            ws.on("play", () => (this.isPlaying = true));
            ws.on("pause", () => (this.isPlaying = false));
            ws.on("timeupdate", (ct) => (this.currentTime = ct));
            ws.on("interaction", () => (this.activeRegion = null));
        });
        this.ws.on("decode", () => {
            if (!this.ws) throw new Error("Can't initialize WaveSurfer");
            const ws = this.ws;
            const regions = this.regions;

            regions.on("region-created", () => {});
            regions.on("region-out", () => {});
            regions.on("region-updated", () => {});
            regions.on("region-clicked", () => {});
        });

        const blob = new Blob([audio], { type: "audio/mp3" });
        this.ws!.loadBlob(blob);
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
