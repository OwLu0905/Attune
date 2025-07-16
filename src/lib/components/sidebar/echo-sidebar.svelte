<script lang="ts">
    import { page } from "$app/state";
    import { getUserContext } from "@/user/userService.svelte";
    import { getAudioListContext } from "@/audio/audioListService.svelte";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import * as HoverCard from "@/components/ui/hover-card/";
    import { PlusCircle } from "@lucide/svelte";
    import Invalid from "@/components/error/invalid.svelte";
    import ErrorMessage from "@/components/error/error-message.svelte";
    import { cn } from "@/utils";
    import { fade } from "svelte/transition";

    const { getUser } = getUserContext();
    const audioApi = getAudioListContext();

    const user = getUser();

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
                {#await audioApi.refreshAudioList(user.accessToken)}
                    <span>
                        <div class="bg-muted/50 aspect-video rounded-xl"></div>
                    </span>
                {:then _}
                    {#each audioApi.audioList as audio (audio.id)}
                        <Sidebar.MenuItem
                            class="min-w-0 group-data-[collapsible=icon]:hidden"
                        >
                            <HoverCard.Root>
                                <HoverCard.Trigger>
                                    <Sidebar.MenuButton
                                        class={cn(
                                            "",
                                            echoId === audio.id &&
                                                "text-primary",
                                        )}
                                    >
                                        {#snippet child({ props })}
                                            <a
                                                href="/echo/{audio.id}"
                                                class="truncate font-mono"
                                                {...props}
                                            >
                                                <span out:fade|global>
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
                    <ErrorMessage />
                {/await}
            </Sidebar.Menu>
        </Sidebar.Group>
    </Sidebar.Content>
{/if}
