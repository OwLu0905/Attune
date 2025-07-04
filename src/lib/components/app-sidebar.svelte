<script lang="ts" module>
    import {
        AudioWaveform,
        BookOpen,
        Bot,
        ChartPie,
        Command,
        Frame,
        GalleryVerticalEnd,
        Map,
        Settings2,
        SquareTerminal,
    } from "@lucide/svelte";
</script>

<script lang="ts">
    import { page } from "$app/state";
    import type { ComponentProps } from "svelte";
    import NavUser from "$lib/components/nav-user.svelte";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import TopicSwitcher from "$lib/components/topic-switcher.svelte";
    import EchoSidebar from "$lib/components/sidebar/echo-sidebar.svelte";
    import NovelSidebar from "$lib/components/sidebar/novel-sidebar.svelte";
    import DefaultSidebar from "$lib/components/sidebar/default-sidebar.svelte";
    import { getUserContext } from "@/user/userService.svelte";

    let { getUser } = getUserContext();
    const user = getUser();

    const data = $derived({
        user: {
            name: user?.name || "Guest",
            email: user?.email || "",
            avatar: user.picture || "",
        },
        topics: [
            {
                name: "Echo",
                logo: GalleryVerticalEnd,
                plan: "Enterprise",
                link: "/echo/1",
            },
            {
                name: "Novel",
                logo: AudioWaveform,
                plan: "Startup",
                link: "/novel",
            },
            {
                name: "Home",
                logo: Command,
                plan: "Free",
                link: "/",
            },
            {
                name: "YouTube",
                logo: Command,
                plan: "Pro",
                link: "/yt",
            },

            {
                name: "Model",
                logo: Command,
                plan: "Pro",
                link: "/model",
            },
        ],
        navMain: [
            {
                title: "Playground",
                url: "#",
                icon: SquareTerminal,
                isActive: true,
                items: [
                    {
                        title: "History",
                        url: "#",
                    },
                    {
                        title: "Starred",
                        url: "#",
                    },
                    {
                        title: "Settings",
                        url: "#",
                    },
                ],
            },
            {
                title: "Models",
                url: "#",
                icon: Bot,
                items: [
                    {
                        title: "Genesis",
                        url: "#",
                    },
                    {
                        title: "Explorer",
                        url: "#",
                    },
                    {
                        title: "Quantum",
                        url: "#",
                    },
                ],
            },
            {
                title: "Documentation",
                url: "#",
                icon: BookOpen,
                items: [
                    {
                        title: "Introduction",
                        url: "#",
                    },
                    {
                        title: "Get Started",
                        url: "#",
                    },
                    {
                        title: "Tutorials",
                        url: "#",
                    },
                    {
                        title: "Changelog",
                        url: "#",
                    },
                ],
            },
            {
                title: "Settings",
                url: "#",
                icon: Settings2,
                items: [
                    {
                        title: "General",
                        url: "#",
                    },
                    {
                        title: "Team",
                        url: "#",
                    },
                    {
                        title: "Billing",
                        url: "#",
                    },
                    {
                        title: "Limits",
                        url: "#",
                    },
                ],
            },
        ],
        projects: [
            {
                name: "Design Engineering",
                url: "#",
                icon: Frame,
            },
            {
                name: "Sales & Marketing",
                url: "#",
                icon: ChartPie,
            },
            {
                name: "Travel",
                url: "#",
                icon: Map,
            },
        ],
    });

    let {
        ref = $bindable(null),
        collapsible = "icon",
        ...restProps
    }: ComponentProps<typeof Sidebar.Root> = $props();

    let url = $derived(page.url);
    let pathname = $derived(url.pathname);
    let isNovel = $derived(pathname.includes("novel"));
    let isEcho = $derived(pathname.includes("echo"));
</script>

<Sidebar.Root bind:ref {collapsible} {...restProps}>
    <Sidebar.Header>
        <TopicSwitcher topics={data.topics} />
    </Sidebar.Header>
    {#if isEcho}
        <EchoSidebar />
    {:else if isNovel}
        <NovelSidebar />
    {:else}
        <DefaultSidebar {data} />
    {/if}
    <Sidebar.Footer>
        <NavUser user={data.user} />
    </Sidebar.Footer>
    <Sidebar.Rail />
</Sidebar.Root>
