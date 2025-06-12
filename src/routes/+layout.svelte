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
                class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12"
            >
                <div class="flex items-center gap-2 px-4">
                    <Sidebar.Trigger class="-ml-1" />
                    <Separator orientation="vertical" class="mr-2 h-4" />
                    <Breadcrumb.Root>
                        <Breadcrumb.List>
                            <Breadcrumb.Item class="hidden md:block">
                                <Breadcrumb.Link href="#"
                                    >Building Your Application</Breadcrumb.Link
                                >
                            </Breadcrumb.Item>
                            <Breadcrumb.Separator class="hidden md:block" />
                            <Breadcrumb.Item>
                                <Breadcrumb.Page>Data Fetching</Breadcrumb.Page>
                            </Breadcrumb.Item>
                        </Breadcrumb.List>
                    </Breadcrumb.Root>
                </div>
            </header>
            <h1 class="p-4 py-2 text-3xl font-bold text-primary">
                {mainRoute}
            </h1>
            <div class="flex flex-1 flex-col gap-4 p-4">
                {#if isHomePage}
                    <div class="grid auto-rows-min gap-4 md:grid-cols-3">
                        <div class="aspect-video rounded-xl bg-muted/50"></div>
                        <div class="aspect-video rounded-xl bg-muted/50"></div>
                        <div class="aspect-video rounded-xl bg-muted/50"></div>
                        <div class="col-span-4 w-full">
                            {@render children()}
                        </div>
                    </div>
                    <div
                        class="min-h-screen flex-1 rounded-xl bg-muted/50 md:min-h-min"
                    ></div>
                {:else}
                    {@render children()}
                {/if}
            </div>
        </Sidebar.Inset>
    </Sidebar.Provider>
{/if}
