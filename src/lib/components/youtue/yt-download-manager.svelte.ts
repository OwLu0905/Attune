import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { UnlistenFn } from "@tauri-apps/api/event";

import {
    DOWNLOAD_STATUS_EVENT,
    DOWNLOAD_YT_EVENT,
    type DownloadStatus,
} from "@/types/yt-download";

import type { DownloadSectionParam } from "./types";

export class YtDownloadManager {
    message = $state("");
    unlisten = $state<UnlistenFn | undefined>(undefined);

    async initialize() {
        this.unlisten = await listen<DownloadStatus>(
            DOWNLOAD_YT_EVENT.status,
            (event) => {
                const status = event.payload;
                console.log(status);
                this.handleStatusUpdate(status);
            },
        );
    }

    get getMessage() {
        return this.message;
    }

    handleStatusUpdate(status: DownloadStatus) {
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

    async handleDownload({ start, end, url }: DownloadSectionParam) {
        await invoke(DOWNLOAD_YT_EVENT.download_section, {
            start,
            end,
            url,
        });
    }

    cleanup() {
        if (this.unlisten) {
            this.unlisten();
            this.unlisten = undefined;
        }
    }
}
