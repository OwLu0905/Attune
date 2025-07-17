<script lang="ts">
    import * as Tabs from "@/components/ui/tabs/";
    import { fade } from "svelte/transition";
    import AppearanceTab from "@/components/settings/appearance-tab.svelte";
    import LanguageTab from "@/components/settings/language-tab.svelte";
    import ProfileTab from "@/components/settings/profile-tab.svelte";
    import { getUserContext } from "@/user/userService.svelte";
    import { commands } from "@/tauri";
    import type { AppSettings } from "@/tauri";
    import ErrorMessage from "@/components/error/error-message.svelte";
    import {
        createAppSettingsContext,
        setAppSettingsContext,
    } from "./app-setting-context.svelte";

    let activeTab = $state("appearance");

    let { getUser } = getUserContext();
    let user = getUser();

    const appSettingsContext = createAppSettingsContext();
    setAppSettingsContext(appSettingsContext);

    async function load() {
        if (!user.accessToken) return;

        const result = await commands.handleGetAppSettings(user.accessToken);

        if (result.status === "error") {
            throw new Error(result.error);
        }
        appSettingsContext.appSettings = result.data;
    }
</script>

<div class="container mx-auto max-w-6xl overflow-auto p-6">
    <h1 class="mb-6 text-3xl font-bold">Settings</h1>

    {#await load() then _}
        {#if appSettingsContext.appSettings}
            <Tabs.Root
                bind:value={activeTab}
                orientation="vertical"
                class="flex w-full flex-row gap-6"
            >
                <Tabs.List class="flex h-fit w-48 flex-col space-y-3 p-2">
                    <Tabs.Trigger
                        value="appearance"
                        class="w-full justify-start px-4 py-3"
                        >Appearance</Tabs.Trigger
                    >
                    <Tabs.Trigger
                        value="language"
                        class="w-full justify-start px-4 py-3"
                        >Language & Model</Tabs.Trigger
                    >
                    <Tabs.Trigger
                        value="profile"
                        class="w-full justify-start px-4 py-3"
                        >Profile</Tabs.Trigger
                    >
                </Tabs.List>

                {#key activeTab}
                    <div class="flex flex-1 flex-row" in:fade>
                        <Tabs.Content value="appearance" class="mt-0">
                            <AppearanceTab />
                        </Tabs.Content>

                        <Tabs.Content value="language" class="mt-0">
                            <LanguageTab />
                        </Tabs.Content>

                        <Tabs.Content value="profile" class="mt-0">
                            <ProfileTab />
                        </Tabs.Content>
                    </div>
                {/key}
            </Tabs.Root>
        {/if}
    {:catch error}
        <ErrorMessage />
    {/await}
</div>
