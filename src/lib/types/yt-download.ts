export const DOWNLOAD_YT_EVENT = {
    status: "download_status",
    download_section: "download_yt_sections",
};

export const DOWNLOAD_STATUS_EVENT = {
    started: "Started",
    progress: "Progress",
    already_downloaded: "AlreadyDownloaded",
    finished: "Finished",
    error: "Error",
} as const;

export type DownloadStatus =
    | { type: typeof DOWNLOAD_STATUS_EVENT.started }
    | { type: typeof DOWNLOAD_STATUS_EVENT.progress; message: string }
    | { type: typeof DOWNLOAD_STATUS_EVENT.already_downloaded }
    | { type: typeof DOWNLOAD_STATUS_EVENT.finished }
    | { type: typeof DOWNLOAD_STATUS_EVENT.error; message: string };
