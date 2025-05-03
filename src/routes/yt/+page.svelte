<script lang="ts">
    import Button from "@/components/ui/button/button.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import type { UnlistenFn } from "@tauri-apps/api/event";
    import { onMount } from "svelte";

    export const DOWNLOAD_STATUS = {
        STARTED: "Started",
        PROGRESS: "Progress",
        ALREADY_DOWNLOADED: "AlreadyDownloaded",
        FINISHED: "Finished",
        ERROR: "Error",
    } as const;

    // Type for the status constants
    type DownloadStatusType =
        (typeof DOWNLOAD_STATUS)[keyof typeof DOWNLOAD_STATUS];

    // Define the status types with their data
    type DownloadStatus =
        | { type: typeof DOWNLOAD_STATUS.STARTED }
        | { type: typeof DOWNLOAD_STATUS.PROGRESS; message: string }
        | { type: typeof DOWNLOAD_STATUS.ALREADY_DOWNLOADED }
        | { type: typeof DOWNLOAD_STATUS.FINISHED }
        | { type: typeof DOWNLOAD_STATUS.ERROR; message: string };

    let message = $state("");

    onMount(() => {
        let unlisten: UnlistenFn | undefined;
        async function download() {
            unlisten = await listen<DownloadStatus>(
                "download_status",
                (event) => {
                    const status = event.payload;

                    switch (status.type) {
                        case DOWNLOAD_STATUS.STARTED:
                            console.log("Download started");
                            message = "Download started";
                            break;
                        case DOWNLOAD_STATUS.PROGRESS:
                            console.log("Download progress:", status.message);

                            console.log("AAA", status?.message ?? "empty");
                            message = `🔥 ${status.message} \r\n`;
                            break;
                        case DOWNLOAD_STATUS.ALREADY_DOWNLOADED:
                            console.log("Video was already downloaded");

                            break;
                        case DOWNLOAD_STATUS.FINISHED:
                            console.log("Download completed");
                            message = "Download completed";
                            break;
                        case DOWNLOAD_STATUS.ERROR:
                            console.error("Download error:", status.message);
                            message = `! ${status.message} \r\n`;
                            break;
                    }
                },
            );
        }
        download();
        // return () => {
        //     if (unlisten) unlisten();
        // };
    });
</script>

<div>
    <Button
        onclick={async () => {
            message = "click";
            try {
                await invoke("download_yt_sections", {
                    url: "https://www.youtube.com/watch?v=v5oi8zj0E7g",
                    start: 0,
                    end: 10,
                });
            } catch (err) {
                if (typeof err === "string") {
                    message = err;
                } else {
                    console.log(err);
                }
            }
        }}>Download</Button
    >
</div>
{message}

<a href="/yt/embed">embed</a>
