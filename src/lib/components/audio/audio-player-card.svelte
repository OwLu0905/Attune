<script lang="ts">
    import { commands } from "$lib/tauri";
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy, onMount } from "svelte";
    import { getUserContext } from "@/user/userService.svelte";

    import * as Card from "$lib/components/ui/card/index.js";

    import { LoaderCircle, Mic } from "@lucide/svelte";
    import { AudioPlayer } from "./audio-player.svelte";
    import ListCard from "./list-card.svelte";

    import { getSubtitleFile } from "@/utils";

    import type { SubtitleSegment } from "./types";
    import type { AudioItem, BookmarkDictationView } from "$lib/tauri";
    import EditorTabContainer from "../editor/editor-tab-container.svelte";
    import AudioDropdown from "./audio-dropdown.svelte";
    import { getAppSettingsContext } from "../../../routes/setting/app-setting-context.svelte";

    interface Props {
        videoPath: Uint8Array<ArrayBuffer>;
        audioItem: AudioItem;
    }

    let { audioItem = $bindable(), videoPath }: Props = $props();

    const { getUser } = getUserContext();
    const user = getUser();

    const appSettingsApi = getAppSettingsContext();

    let audioPlayer: AudioPlayer | undefined = $state.raw(undefined);
    let container: HTMLElement | undefined = $state.raw(undefined);
    let volume = $state(10);
    let subtitles: SubtitleSegment[] = $state([]);

    let isCheckingHealth = $state(false);
    let isTranscribing = $state(false);

    let videoRef: HTMLVideoElement;
    let videoUrl = $derived.by(() => {
        const blob = new Blob([videoPath], { type: "video/mp4" });
        return URL.createObjectURL(blob);
    });

    let dictationId = $state(0);
    let prog = $state("");

    let combinedList = $state<BookmarkDictationView[]>([]);

    let dictationItem = $derived.by(() => {
        if (subtitles.length > 0) {
            return subtitles?.[dictationId] || undefined;
        }
        return undefined;
    });

    async function checkModelHealthy() {
        isCheckingHealth = true;
        try {
            const result = await commands.checkModelHealth();
            return result.status === "ok" ? result.data : false;
        } catch (error) {
            return false;
        } finally {
            isCheckingHealth = false;
        }
    }

    async function getSubtitle() {
        try {
            if (!user.accessToken) {
                throw new Error("User not authenticated");
            }

            isTranscribing = true;

            const isHealthy = await checkModelHealthy();
            if (!isHealthy) {
                isTranscribing = false;
                return;
            }

            // TODO: read from setting
            const transcribe_result =
                await commands.startTranscribeServiceStreaming(
                    audioItem.id,
                    appSettingsApi?.appSettings?.selectedModel ?? "small.en",
                );

            if (transcribe_result.status === "error") {
                throw new Error(transcribe_result.error);
            }
            const update_result = await commands.handleUpdateAudioTranscribe(
                user.accessToken,
                audioItem.id,
            );

            if (update_result.status === "error") {
                throw new Error(update_result.error);
            }

            audioItem = update_result.data;
            subtitles = await getSubtitleFile(audioItem.id);
        } catch (error) {
            console.error(error);
        } finally {
            isTranscribing = false;
        }
    }

    async function onPlaySection(start: number, end: number, setEnd?: boolean) {
        if (!audioPlayer) return;
        audioPlayer.onPlaySection(start, end, setEnd);
    }
    async function onPause() {
        if (!audioPlayer) return;
        audioPlayer.onPause();
    }

    onMount(() => {
        async function load() {
            if (!container) return;
            if (audioPlayer) {
                audioPlayer.destroy();
            }
            audioPlayer = new AudioPlayer(container, videoRef);

            await audioPlayer.initialize(undefined, () => {
                if (!audioPlayer) return;
                volume = audioPlayer.getVolume() * 100;
            });

            if (videoPath && "mediaSession" in navigator) {
                navigator.mediaSession.metadata = new MediaMetadata({
                    title: audioItem.id,
                });

                // Set up media controls to work with WaveSurfer
                navigator.mediaSession.setActionHandler("play", () => {
                    audioPlayer?.onPlay();
                });
            }

            if (audioItem.transcribe === 1) {
                subtitles = await getSubtitleFile(audioItem.id);
            }
        }
        load();

        return () => {
            if (audioPlayer) {
                audioPlayer.destroy();
            }
        };
    });

    onMount(() => {
        listen("transcription-progress", (event) => {
            // TODO: fix types
            const { status, message, progress } = event.payload as {
                status: string;
                message: string;
                progress: string;
            };
            prog = message;
        });
    });

    onDestroy(() => {
        if (videoUrl) {
            URL.revokeObjectURL(videoUrl);
        }
    });
</script>

<Card.Root class="h-full">
    <Card.Header class="flex flex-row justify-between">
        <div class="flex flex-col gap-1.5">
            <Card.Title>{audioItem.title}</Card.Title>
            <Card.Description class="text-xs tabular-nums"
                >{audioItem.id} - {audioItem.url}</Card.Description
            >
        </div>
        <AudioDropdown
            isTranscribed={audioItem.transcribe === 1 && !isTranscribing}
            {isTranscribing}
            {getSubtitle}
        />
    </Card.Header>

    <Card.Content class="@container shrink grow overflow-hidden">
        <div class="flex h-full flex-col gap-6 py-6 @3xl:flex-row">
            <div class="w-full shrink-0 @3xl:w-120 @4xl:w-1/2">
                <!-- svelte-ignore a11y_media_has_caption -->
                <video
                    bind:this={videoRef}
                    src={videoUrl}
                    controls
                    playsinline
                    class="rounded-xl"
                    style="width: 480px; height: 270px; margin: 4px auto; display: block;"
                >
                </video>

                <div
                    bind:this={container}
                    style="width:480px; height: 32px; margin:auto"
                ></div>

                {#if audioPlayer && dictationItem}
                    <EditorTabContainer
                        audioId={audioItem.id}
                        bind:dictationId
                        bind:combinedList
                        length={subtitles.length}
                        {dictationItem}
                        {audioPlayer}
                        {onPause}
                        {onPlaySection}
                    />
                {/if}
            </div>
            {#if audioItem.transcribe === 1}
                <ListCard
                    bind:dictationId
                    bind:combinedList
                    {audioItem}
                    {subtitles}
                    {audioPlayer}
                    {onPause}
                    {onPlaySection}
                />
            {/if}

            {#if audioItem.transcribe === 0 && isTranscribing}
                <LoaderCircle
                    class="text-primary m-auto h-12 w-12 animate-spin"
                />
                {prog}
            {/if}
        </div>
    </Card.Content>

    <Card.Footer
        class="flex w-full shrink-0 basis-10 items-center justify-center gap-2 border-t"
    >
        <div class="flex gap-2">
            <div class="flex items-center gap-1">
                <div class="bg-primary/60 h-2 w-2 rounded-full"></div>
                <div class="text-xs">Current Segment</div>
            </div>
            <div class="text-xs tabular-nums">
                {audioPlayer?.currentTime?.toFixed(2)} (sec)
            </div>
        </div>
        <Mic class="h-4 w-4" />
    </Card.Footer>
</Card.Root>
