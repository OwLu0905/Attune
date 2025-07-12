<script lang="ts">
    import { page } from "$app/state";
    import { getUserContext } from "@/user/userService.svelte";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import * as HoverCard from "@/components/ui/hover-card/";
    import { PlusCircle } from "@lucide/svelte";
    import Invalid from "@/components/error/invalid.svelte";
    import Error from "@/components/error/error.svelte";
    import { cn } from "@/utils";
    import { getAudioList } from "@/audio";
    import type { AudioListItem } from "$lib/tauri";

    const { getUser } = getUserContext();

    const user = getUser();

    let audioList: AudioListItem[] = $state([]);

    async function getList(token: string) {
        audioList = await getAudioList(token);
    }

    let params = $derived(page.params);
    let echoId = $derived(params["id"]);

    // TODO: group by created time
</script>

{#if !user.accessToken}
    <Invalid />
{:else}
    <Sidebar.Content>
        <Sidebar.Group>
            <Sidebar.Menu class="">
                <Sidebar.MenuItem class="flex justify-center">
                    <Sidebar.MenuButton
                        variant="default"
                        class="text-primary font-bold"
                    >
                        {#snippet child({ props })}
                            <a href="/yt/create" {...props}>
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
                {#await getList(user.accessToken)}
                    <span>
                        <div class="bg-muted/50 aspect-video rounded-xl"></div>
                    </span>
                {:then _}
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
                {:catch error}
                    <Error />
                {/await}
            </Sidebar.Menu>
        </Sidebar.Group>
    </Sidebar.Content>
{/if}
