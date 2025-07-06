<script lang="ts">
    import { page } from "$app/state";
    import { invoke } from "@tauri-apps/api/core";
    import { getUserContext } from "@/user/userService.svelte";
    import { cn } from "@/utils";
    import { PlusCircle } from "@lucide/svelte";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import * as HoverCard from "@/components/ui/hover-card/";

    import type { AudioItem } from "@/types/audio";

    const { getUser } = getUserContext();

    const user = getUser();

    let audioList: AudioItem[] = $state([]);

    $effect(() => {
        async function getAudioList() {
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
        }

        getAudioList();
    });

    let params = $derived(page.params);
    let echoId = $derived(params["id"]);

    // TODO: group by created time
</script>

<Sidebar.Content>
    <Sidebar.Group>
        <Sidebar.Menu class="">
            <Sidebar.MenuItem class="flex justify-center">
                <Sidebar.MenuButton
                    variant="default"
                    class="text-primary font-bold"
                >
                    {#snippet child({ props })}
                        <a href="/yt/" {...props}>
                            <PlusCircle strokeWidth={2.5} />

                            <span>New Audio</span>
                        </a>
                    {/snippet}
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>
        </Sidebar.Menu>
    </Sidebar.Group>
    <Sidebar.Group>
        <Sidebar.GroupLabel>List</Sidebar.GroupLabel>

        <Sidebar.Menu>
            {#each audioList as audio (audio.id)}
                <Sidebar.MenuItem
                    class="min-w-0 group-data-[collapsible=icon]:hidden"
                >
                    <HoverCard.Root>
                        <HoverCard.Trigger>
                            <Sidebar.MenuButton
                                class={cn(
                                    "",
                                    echoId === audio.id &&
                                        "!text-sidebar-primary",
                                )}
                            >
                                {#snippet child({ props })}
                                    <a
                                        href="/echo/{audio.id}"
                                        class="truncate font-mono"
                                        {...props}
                                    >
                                        <span>
                                            {audio.title}
                                        </span>
                                    </a>
                                {/snippet}
                            </Sidebar.MenuButton>
                        </HoverCard.Trigger>
                        <HoverCard.Content side="right">
                            <img
                                class="aspect-video h-20 object-cover"
                                src={audio.thumbnail}
                                alt={audio.title}
                            />
                        </HoverCard.Content>
                    </HoverCard.Root>
                </Sidebar.MenuItem>
            {/each}
        </Sidebar.Menu>
    </Sidebar.Group>
</Sidebar.Content>
