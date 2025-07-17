<script lang="ts">
    import { commands } from "$lib/tauri";
    import { getUserContext } from "@/user/userService.svelte";
    import { getAudioListContext } from "@/audio/audioListService.svelte";
    import Button from "@/components/ui/button/button.svelte";
    import Badge from "@/components/ui/badge/badge.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as AlertDialog from "@/components/ui/alert-dialog";
    import Invalid from "@/components/error/invalid.svelte";
    import ErrorMessage from "@/components/error/error-message.svelte";
    import { Trash, Upload } from "@lucide/svelte";
    import { fade } from "svelte/transition";

    const { getUser } = getUserContext();
    const audioApi = getAudioListContext();

    const user = getUser();
</script>

{#if !user.accessToken}
    <Invalid />
{:else}
    <a href="/yt/create">
        <Button variant="outline">
            <Upload />
            Add new audio
        </Button>
    </a>

    <div class="@container grid grid-cols-12 gap-2">
        {#await audioApi.refreshAudioList(user.accessToken)}
            <span>
                <div class="bg-muted/50 aspect-video rounded-xl"></div>
            </span>
        {:then _}
            {#each audioApi.audioList as audio (audio.id)}
                <Card.Root
                    class="col-span-12 @sm:col-span-6 @xl:col-span-4 @3xl:col-span-3"
                >
                    <Card.Header>
                        <img
                            out:fade
                            class="aspect-video h-full w-full object-cover"
                            src={audio.thumbnail}
                            alt={audio.title}
                        />
                    </Card.Header>
                    <Card.Content>
                        <div class="flex flex-col gap-1">
                            <a
                                class="truncate text-base"
                                href="/echo/{audio.id}"
                            >
                                {audio.title}
                            </a>
                            <p
                                class="text-secondary-foreground truncate text-sm"
                            >
                                {audio.description || ""}
                            </p>
                            <div>
                                {#if audio.transcribe === 1}
                                    <Badge variant="outline">
                                        <span>Done</span>
                                    </Badge>
                                {:else}
                                    <Badge variant="outline">
                                        <span>Uninitialize</span>
                                    </Badge>
                                {/if}
                            </div>
                        </div>
                    </Card.Content>
                    <Card.Footer>
                        <AlertDialog.Root>
                            <AlertDialog.Trigger>
                                <Button size="sm" variant="outline">
                                    <Trash class="text-destructive w-4" />
                                </Button>
                            </AlertDialog.Trigger>
                            <AlertDialog.Content>
                                <AlertDialog.Header>
                                    <AlertDialog.Title
                                        >Are you absolutely sure?</AlertDialog.Title
                                    >
                                    <AlertDialog.Description>
                                        This action cannot be undone. This will
                                        permanently delete the audio and remove
                                        your data from our servers.
                                    </AlertDialog.Description>
                                </AlertDialog.Header>
                                <AlertDialog.Footer>
                                    <AlertDialog.Cancel
                                        >Cancel</AlertDialog.Cancel
                                    >
                                    <AlertDialog.Action
                                        onclick={async () => {
                                            if (!user.accessToken) return;

                                            try {
                                                const result =
                                                    await commands.handleDeleteAudio(
                                                        user.accessToken,
                                                        audio.id,
                                                    );

                                                if (result.status === "error") {
                                                    throw new Error(
                                                        result.error,
                                                    );
                                                }

                                                audioApi.removeAudioItem(
                                                    audio.id,
                                                );
                                            } catch (error) {
                                                console.error(error);
                                            }
                                        }}>Continue</AlertDialog.Action
                                    >
                                </AlertDialog.Footer>
                            </AlertDialog.Content>
                        </AlertDialog.Root>
                        <span class="ml-auto text-xs">
                            {audio.lastUsedAt}
                        </span>
                    </Card.Footer>
                </Card.Root>
            {/each}
        {:catch error}
            <ErrorMessage />
        {/await}
    </div>
{/if}
