<script lang="ts">
    import { commands } from "$lib/tauri";
    import * as Avatar from "$lib/components/ui/avatar/index.js";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import { useSidebar } from "$lib/components/ui/sidebar/index.js";
    import { getUserContext } from "@/user/userService.svelte";
    import { LogIn, Settings } from "@lucide/svelte";
    import BadgeCheck from "@lucide/svelte/icons/badge-check";
    import ChevronsUpDown from "@lucide/svelte/icons/chevrons-up-down";
    import LogOut from "@lucide/svelte/icons/log-out";

    let { user }: { user: { name: string; email: string; avatar: string } } =
        $props();
    const sidebar = useSidebar();

    const { getUser, setUser } = getUserContext();
    const userInfo = getUser();

    async function logout() {
        try {
            const result = await commands.logoutUser();

            if (result.status === "error") {
                throw new Error(result.error);
            }
        } catch (error) {
            console.error(error);
        } finally {
            setUser({
                userId: null,
                accessToken: null,
                name: null,
                email: null,
                picture: null,
            });
        }
    }
</script>

<Sidebar.Menu>
    <Sidebar.MenuItem>
        <DropdownMenu.Root>
            <DropdownMenu.Trigger>
                {#snippet child({ props })}
                    <Sidebar.MenuButton
                        size="lg"
                        class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                        {...props}
                    >
                        <Avatar.Root class="h-8 w-8 rounded-lg">
                            <Avatar.Image src={user.avatar} alt={user.name} />
                            <Avatar.Fallback class="rounded-lg"
                                >Hi</Avatar.Fallback
                            >
                        </Avatar.Root>
                        <div
                            class="grid flex-1 text-left text-sm leading-tight"
                        >
                            <span class="truncate font-semibold"
                                >{user.name}</span
                            >
                            <span class="truncate text-xs">{user.email}</span>
                        </div>
                        <ChevronsUpDown class="ml-auto size-4" />
                    </Sidebar.MenuButton>
                {/snippet}
            </DropdownMenu.Trigger>
            <DropdownMenu.Content
                class="w-(--bits-dropdown-menu-anchor-width) min-w-56 rounded-lg"
                side={sidebar.isMobile ? "bottom" : "right"}
                align="end"
                sideOffset={4}
            >
                <DropdownMenu.Label class="p-0 font-normal">
                    <div
                        class="flex items-center gap-2 px-1 py-1.5 text-left text-sm"
                    >
                        <Avatar.Root class="h-8 w-8 rounded-lg">
                            <Avatar.Image src={user.avatar} alt={user.name} />
                            <Avatar.Fallback class="rounded-lg"
                                >Hi</Avatar.Fallback
                            >
                        </Avatar.Root>
                        <div
                            class="grid flex-1 text-left text-sm leading-tight"
                        >
                            <span class="truncate font-semibold"
                                >{user.name}</span
                            >
                            <span class="truncate text-xs">{user.email}</span>
                        </div>
                    </div>
                </DropdownMenu.Label>
                <DropdownMenu.Separator />
                <DropdownMenu.Group>
                    <DropdownMenu.Item>
                        <BadgeCheck />
                        Account
                    </DropdownMenu.Item>
                    <DropdownMenu.Item>
                        {#snippet child({ props })}
                            <a href="/setting" {...props}>
                                <Settings />
                                Settings
                            </a>
                        {/snippet}
                    </DropdownMenu.Item>
                </DropdownMenu.Group>
                <DropdownMenu.Separator />

                {#if !userInfo.accessToken}
                    <a href="/login">
                        <DropdownMenu.Item>
                            <LogIn />
                            Log In
                        </DropdownMenu.Item>
                    </a>
                {:else}
                    <DropdownMenu.Item
                        onclick={() => {
                            logout();
                        }}
                    >
                        <LogOut />
                        Log Out
                    </DropdownMenu.Item>
                {/if}
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </Sidebar.MenuItem>
</Sidebar.Menu>
