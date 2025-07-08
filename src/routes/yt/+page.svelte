<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { getUserContext } from "@/user/userService.svelte";
    import Button from "@/components/ui/button/button.svelte";
    import Badge from "@/components/ui/badge/badge.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as AlertDialog from "@/components/ui/alert-dialog";
    import Invalid from "@/components/error/invalid.svelte";
    import Error from "@/components/error/error.svelte";
    import { Trash, Upload } from "@lucide/svelte";
    import { getAudioList } from "$lib/audio.js";
    import type { AudioItem } from "@/types/audio";

    const { getUser } = getUserContext();

    const user = getUser();

    let audioList: AudioItem[] = $state([]);

    async function getList(token: string) {
        audioList = await getAudioList(token);
    }
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
        {#await getList(user.accessToken)}
            <span>
                <div class="bg-muted/50 aspect-video rounded-xl"></div>
            </span>
        {:then _}
            {#each audioList as audio (audio.id)}
                <Card.Root
                    class="col-span-12 @sm:col-span-6 @xl:col-span-4 @3xl:col-span-3"
                >
                    <Card.Header>
                        <img
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
                                {audio.description || "none"}
                            </p>
                            <div>
                                {#if audio.transcribe === 1}
                                    <Badge variant="outline">
                                        <span>script</span>
                                    </Badge>
                                {/if}
                            </div>
                        </div>
                    </Card.Content>
                    <Card.Footer>
                        <AlertDialog.Root>
                            <AlertDialog.Trigger>
                                <Trash class="text-destructive w-4" />
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
                                            try {
                                                audioList = await invoke(
                                                    "handle_delete_audio",
                                                    {
                                                        token: user.accessToken,
                                                        audio_id: audio.id,
                                                    },
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
            <Error />
        {/await}
    </div>
{/if}
