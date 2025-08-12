<script>
    import { commands } from "@/tauri";
    import {
        createAppSettingsContext,
        setAppSettingsContext,
    } from "../../setting/app-setting-context.svelte";
    import { getUserContext } from "@/user/userService.svelte";

    let { children } = $props();

    const { getUser } = getUserContext();
    const user = getUser();
    const appSettingsContext = createAppSettingsContext();
    setAppSettingsContext(appSettingsContext);

    async function loadAppSettings() {
        if (!user.accessToken) return;

        const result = await commands.handleGetAppSettings(user.accessToken);

        if (result.status === "error") {
            throw new Error(result.error);
        }
        appSettingsContext.appSettings = result.data;
    }
</script>

{#await loadAppSettings() then _}
    {@render children()}
{/await}
