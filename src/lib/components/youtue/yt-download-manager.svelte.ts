import { listen } from "@tauri-apps/api/event";
import type { UnlistenFn } from "@tauri-apps/api/event";
import type { DownloadStatus } from "@/types/yt-download";
import { DOWNLOAD_STATUS_EVENT, DOWNLOAD_YT_EVENT } from "@/types/yt-download";

export class YtDownloadManager {
    message = $state("");
    unlisten = $state<UnlistenFn | undefined>(undefined);

    async initialize() {
        this.unlisten = await listen<DownloadStatus>(
            DOWNLOAD_YT_EVENT.download_section,
            (event) => {
                const status = event.payload;
                this.handdleStatusUpdate(status);
            },
        );
    }

    handdleStatusUpdate(status: DownloadStatus) {
        switch (status.type) {
            case DOWNLOAD_STATUS_EVENT.started:
                this.message = "Download Started";
                break;

            case DOWNLOAD_STATUS_EVENT.progress:
                this.message = `🔥 ${status.message ?? ""} \r\n`;
                break;

            case DOWNLOAD_STATUS_EVENT.already_downloaded:
                this.message = "Video was already downloaded";
                break;
            case DOWNLOAD_STATUS_EVENT.finished:
                this.message = "Download completed";
                break;
            case DOWNLOAD_STATUS_EVENT.error:
                this.message = `! ${status.message ?? ""} \r\n`;
            default:
                this.message = "";
        }
    }

    cleanup() {
        if (this.unlisten) {
            this.unlisten();
        }
    }
}
