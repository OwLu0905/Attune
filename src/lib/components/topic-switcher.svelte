<script lang="ts">
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import { useSidebar } from "$lib/components/ui/sidebar/index.js";
    import { ChevronsUpDown, Plus } from "@lucide/svelte";

    let {
        topics,
    }: { topics: { name: string; logo: any; plan: string; link: string }[] } =
        $props();
    const sidebar = useSidebar();

    let activeTopic = $state(topics[0]);
</script>

<Sidebar.Menu>
    <Sidebar.MenuItem>
        <DropdownMenu.Root>
            <DropdownMenu.Trigger>
                {#snippet child({ props })}
                    <Sidebar.MenuButton
                        {...props}
                        size="lg"
                        class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                    >
                        <div
                            class="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground"
                        >
                            <activeTopic.logo class="size-4" />
                        </div>
                        <div
                            class="grid flex-1 text-left text-sm leading-tight"
                        >
                            <span class="truncate font-semibold">
                                {activeTopic.name}
                            </span>
                            <span class="truncate text-xs"
                                >{activeTopic.plan}</span
                            >
                        </div>
                        <ChevronsUpDown class="ml-auto" />
                    </Sidebar.MenuButton>
                {/snippet}
            </DropdownMenu.Trigger>
            <DropdownMenu.Content
                class="w-[--bits-dropdown-menu-anchor-width] min-w-56 rounded-lg"
                align="start"
                side={sidebar.isMobile ? "bottom" : "right"}
                sideOffset={4}
            >
                <DropdownMenu.Label class="text-xs text-muted-foreground"
                    >topics</DropdownMenu.Label
                >
                {#each topics as topic, index (topic.name)}
                    <a href={topic.link}>
                        <DropdownMenu.Item
                            onSelect={() => (activeTopic = topic)}
                            class="gap-2 p-2"
                        >
                            <div
                                class="flex size-6 items-center justify-center rounded-sm border"
                            >
                                <topic.logo class="size-4 shrink-0" />
                            </div>
                            {topic.name}
                            <DropdownMenu.Shortcut
                                >⌘{index + 1}</DropdownMenu.Shortcut
                            >
                        </DropdownMenu.Item>
                    </a>
                {/each}
                <DropdownMenu.Separator />
                <DropdownMenu.Item class="gap-2 p-2">
                    <div
                        class="flex size-6 items-center justify-center rounded-md border bg-background"
                    >
                        <Plus class="size-4" />
                    </div>
                    <div class="font-medium text-muted-foreground">
                        Add topic
                    </div>
                </DropdownMenu.Item>
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </Sidebar.MenuItem>
</Sidebar.Menu>
