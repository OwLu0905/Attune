<script lang="ts">
    import "../app.css";
    import { page } from "$app/state";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    import AppSidebar from "$lib/components/app-sidebar.svelte";
    import { Separator } from "$lib/components/ui/separator";
    import { Toaster } from "$lib/components/ui/sonner/index.js";
    import * as Sidebar from "$lib/components/ui/sidebar";
    import * as Breadcrumb from "$lib/components/ui/breadcrumb";
    import {
        setUserContext,
        type UserInfo,
    } from "$lib/user/userService.svelte";

    let { children } = $props();
    let url = $derived(page.url);

    let user: UserInfo = $state({
        userId: null,
        accessToken: null,
        name: null,
        email: null,
        picture: null,
    });

    setUserContext({
        getUser: () => user,
        setUser: (userData) => {
            user.userId = userData.userId;
            user.accessToken = userData.accessToken;
            user.name = userData.name;
            user.email = userData.email;
            user.picture = userData.picture;
        },
    });

    onMount(async () => {
        try {
            const userData: UserInfo = await invoke("check_persist_user");
            user.userId = userData.userId;
            user.accessToken = userData.accessToken;
            user.name = userData.name;
            user.email = userData.email;
            user.picture = userData.picture;
        } catch (error) {
            console.error("Guest");
        }
    });

    let mainRoute = $derived.by(() => {
        const name =
            url.pathname.split("/").filter((segment) => segment !== "")[0] ??
            "Owlet";
        const title = name.charAt(0).toUpperCase() + name.slice(1);
        return title;
    });

    let pathname = $derived(url.pathname);
    let isHomePage = $derived(pathname === "/");
    let isAuth = $derived(pathname.includes("/login"));
</script>

{#if isAuth && !user.accessToken}
    <div class="flex flex-col gap-4">
        {@render children()}
    </div>
{:else}
    <Toaster />
    <Sidebar.Provider>
        <AppSidebar />
        <Sidebar.Inset>
            <header
                class="mx-4 flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12"
            >
                <div class="ml-2 flex items-center">
                    <Sidebar.Trigger class="-ml-1" />
                </div>
                <h1 class="text-primary py-2 text-3xl font-bold">
                    {mainRoute}
                </h1>
            </header>
            <div
                class="flex h-0 shrink grow flex-col gap-4 overflow-hidden p-4"
            >
                {#if isHomePage}
                    <div class="grid auto-rows-min gap-4 md:grid-cols-3">
                        <div class="bg-muted/50 aspect-video rounded-xl"></div>
                        <div class="bg-muted/50 aspect-video rounded-xl"></div>
                        <div class="bg-muted/50 aspect-video rounded-xl"></div>
                        <div class="col-span-4 w-full">
                            {@render children()}
                        </div>
                    </div>
                    <div
                        class="bg-muted/50 min-h-screen flex-1 rounded-xl md:min-h-min"
                    ></div>
                {:else}
                    {@render children()}
                {/if}
            </div>
        </Sidebar.Inset>
    </Sidebar.Provider>
{/if}
