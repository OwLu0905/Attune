import WaveSurfer from "wavesurfer.js";
import RegionsPlugin from "wavesurfer.js/dist/plugins/regions.esm.js";
import RecordPlugin from "wavesurfer.js/dist/plugins/record.esm.js";
import type { Region } from "wavesurfer.js/dist/plugins/regions.esm.js";

import { writeFile, mkdir, BaseDirectory } from "@tauri-apps/plugin-fs";
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

    currentTime = $state("00:00:00");

    isPlaying = $state(false);
    isReady = $state(false);
    isRecording = $state(false);
    blobData: Blob | null = $state(null);
    recordedUrl: string | null = $state(null);

    computedStyle = getComputedStyle(document.documentElement);
    primaryOklch = this.computedStyle.getPropertyValue("--primary").trim();
    secondaryOklch = this.computedStyle.getPropertyValue("--secondary").trim();
    destructiveOklch = this.computedStyle
        .getPropertyValue("--destructive")
        .trim();

    constructor() {}
    async createRecord(container: HTMLElement) {
        if (this.ws) {
            this.ws.destroy();
        }

        if (this.recordedUrl) {
            URL.revokeObjectURL(this.recordedUrl);
            this.recordedUrl = null;
            this.blobData = null;
        }

        this.ws = WaveSurfer.create({
            container,
            progressColor: `${this.destructiveOklch}`,
            waveColor: `${this.destructiveOklch}`,
            barWidth: 2,
            barGap: 1,
            height: 64,
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
        this.record.on("record-progress", (time) => {
            this.currentTime = [
                Math.floor((time % 3600000) / 60000), // minutes
                Math.floor((time % 60000) / 1000), // seconds
                Math.floor((time % 1000) / 10), // milliseconds (in centiseconds, 0-99)
            ]
                .map((v) => (v < 10 ? "0" + v : v))
                .join(":");
        });

        this.record.on("record-end", (blob) => {
            this.isRecording = false;
            if (this.ws) {
                this.ws.destroy();
            }
            this.blobData = blob;
            this.recordedUrl = URL.createObjectURL(blob);
            this.ws = WaveSurfer.create({
                container,
                progressColor: `${this.primaryOklch}`,
                waveColor: `${this.secondaryOklch}`,
                barWidth: 2,
                barGap: 1,
                height: 64,
                backend: WAVESURFER_BACKEND,
                url: this.recordedUrl,
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
    async onSaveFile(index: number, id: string) {
        if (!this.blobData || !this.recordedUrl) return;

        const arrayBuffer = await this.blobData.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);
        const fileExtension =
            this.blobData.type.split(";")[0].split("/")[1] || "webm";

        const uuid = this.recordedUrl.slice(
            this.recordedUrl.lastIndexOf("/") + 1,
        ); // Extract UUID from existing URL

        const dirPath = `data/${id}/${index}/record`;
        try {
            await mkdir(dirPath, {
                baseDir: BaseDirectory.AppLocalData,
                recursive: true, // This creates parent directories if they don't exist
            });
        } catch (error) {
            // Directory might already exist, that's okay
            console.log("Directory creation info:", error);
        }

        await writeFile(
            `data/${id}/${index}/record/${uuid}.${fileExtension}`,
            uint8Array,
            {
                baseDir: BaseDirectory.AppLocalData,
            },
        );
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
    empty() {
        this.currentTime = "00:00:00";
        this.ws?.empty();
    }
    destroy() {
        this.currentTime = "00:00:00";
        this.ws?.destroy();
    }

    deleteRecord() {
        if (this.recordedUrl) {
            URL.revokeObjectURL(this.recordedUrl);
            this.recordedUrl = null;
        }
        this.blobData = null;
        this.empty();
    }
}
