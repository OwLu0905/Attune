<script>
    import { invoke } from "@tauri-apps/api/core";
    import Button from "@/components/ui/button/button.svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import { getUserContext } from "@/user/userService.svelte";
    import { Upload } from "@lucide/svelte";
    import Badge from "@/components/ui/badge/badge.svelte";

    const { getUser } = getUserContext();

    const user = getUser();

    let audioList = $state([]);

    $effect(async () => {
        try {
            if (!user.accessToken) {
                return;
            }
            audioList = await invoke("handle_get_audio_list", {
                token: user.accessToken,
            });
        } catch (error) {
            console.error(error);
        }
    });

    $inspect(audioList);
</script>

<a href="/yt/create">
    <Button variant="outline">
        <Upload />
        Add new audio
    </Button>
</a>

<div class="grid grid-cols-12 gap-2 @container">
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
                    <p class="truncate text-base">
                        {audio.title}
                    </p>
                    <p class="truncate text-sm text-secondary-foreground">
                        {audio.description || "no"}
                    </p>
                    <div>
                        {#if audio.transcribe === 0}
                            <Badge variant="outline">
                                <span>script</span>
                            </Badge>
                        {/if}
                    </div>
                </div>
            </Card.Content>
            <Card.Footer>
                <span></span>
                <span class="ml-auto text-xs">
                    {audio.lastUsedAt}
                </span>
            </Card.Footer>
        </Card.Root>
    {/each}
</div>
