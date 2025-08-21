import { BaseDirectory, mkdir, writeFile } from "@tauri-apps/plugin-fs";

type SupportedMimeType =
    | "audio/webm"
    | "audio/webm;codecs=opus"
    | "audio/mp4"
    | "audio/ogg;codecs=opus";

export class AudioRecorderSimple {
    isRecording = $state(false);
    isReady = $state(false);

    blobData = $state<Blob | null>(null);
    recordedUrl = $state<string | null>(null);
    duration = $state<number>(0);
    currentTime = $state("00:00");

    private mediaRecorder: MediaRecorder | null = null;

    private chunks: Blob[] = [];
    private stream: MediaStream | null = null;
    private mimeType: SupportedMimeType = "audio/webm";

    private constraints: MediaTrackConstraints = {
        echoCancellation: true,
    };

    private recordingInterval: number | null = null;

    constructor() {
        if (MediaRecorder.isTypeSupported("audio/webm;codecs=opus")) {
            this.mimeType = "audio/webm;codecs=opus";
        } else if (MediaRecorder.isTypeSupported("audio/mp4")) {
            this.mimeType = "audio/mp4";
        } else if (MediaRecorder.isTypeSupported("audio/ogg;codecs=opus")) {
            this.mimeType = "audio/ogg;codecs=opus";
        }
    }

    async onRecord(deviceId?: ConstrainDOMString): Promise<void> {
        if (!navigator.mediaDevices.getUserMedia) {
            console.error(
                "The mediaDevices.getUserMedia() method is NOT supported.",
            );
            return;
        }
        this.deleteRecord();

        if (deviceId) {
            this.constraints = {
                ...this.constraints,
                deviceId: deviceId,
            };
        }

        try {
            navigator.mediaDevices
                .getUserMedia({ audio: this.constraints })
                .then(this._onSuccess.bind(this), this._onError.bind(this));
        } catch (err) {
            console.error("onRecord error", err);
        }
    }

    onFinish() {
        if (this.mediaRecorder && this.mediaRecorder.state !== "inactive") {
            this.mediaRecorder.stop();
            if (this.recordingInterval) {
                clearInterval(this.recordingInterval);
                this.recordingInterval = null;
            }
        }
    }

    private async _onSuccess(stream: MediaStream) {
        this.mediaRecorder = new MediaRecorder(stream, {
            mimeType: this.mimeType,
        });
        this.chunks = [];

        this.mediaRecorder.ondataavailable = (e) => {
            // NOTE: should check data.size when using timeslice
            this.chunks.push(e.data);
        };

        this.mediaRecorder.onstop = () => {
            this.isRecording = false;
            this.blobData = new Blob(this.chunks, { type: this.mimeType });
            this.chunks = [];
            this.recordedUrl = window.URL.createObjectURL(this.blobData);

            // NOTE: should I call this line here?
            this.stream?.getTracks().forEach((track) => track.stop());
        };

        this._start();
        this.count();
    }

    private async _onError(err: ErrorEvent) {
        console.log("The following error occured: " + err);
    }

    private async _start() {
        if (!this.mediaRecorder) return;
        this.mediaRecorder.start();
        this.isRecording = true;
    }

    count() {
        let recordingTime = 0;
        this.recordingInterval = window.setInterval(() => {
            recordingTime += 1;
            this.currentTime = this._formatTime(recordingTime);
        }, 1000);
    }

    private _formatTime(seconds: number): string {
        const mins = Math.floor(seconds / 60);
        const secs = Math.floor(seconds % 60);
        return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
    }

    async deleteRecord() {
        if (this.recordedUrl) {
            URL.revokeObjectURL(this.recordedUrl);
            this.recordedUrl = null;
        }
        this.currentTime = "00:00";
    }

    async onSaveFile(id: string, index: number) {
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
    destroy() {
        if (this.recordedUrl) {
            URL.revokeObjectURL(this.recordedUrl);
        }

        if (this.recordingInterval) {
            clearInterval(this.recordingInterval);
            this.recordingInterval = null;
        }

        if (this.stream) {
            this.stream
                .getTracks()
                .forEach((track: MediaStreamTrack) => track.stop());
        }
    }
}
