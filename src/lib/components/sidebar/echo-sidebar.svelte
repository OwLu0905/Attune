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
    import EchoSidebarItem from "./echo-sidebar-item.svelte";

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
            {#await audioApi.refreshAudioList(user.accessToken)}
                <span>
                    <div class="bg-muted/50 aspect-video rounded-xl"></div>
                </span>
            {:then _}
                <EchoSidebarItem {echoId} audioList={audioApi.audioList} />
            {:catch error}
                <ErrorMessage />
            {/await}
        </Sidebar.Group>
    </Sidebar.Content>
{/if}
