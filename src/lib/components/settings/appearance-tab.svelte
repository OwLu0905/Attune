<script lang="ts">
    import { Card } from "@/components/ui/card/";
    import { Label } from "@/components/ui/label/";
    import ThemeSwitch from "@/components/theme-switch.svelte";
    import { commands } from "@/tauri";
    import { getUserContext } from "@/user/userService.svelte";
    import { userPrefersMode, setMode } from "mode-watcher";

    import { getAppSettingsContext } from "../../../routes/setting/app-setting-context.svelte";
    import { onMount } from "svelte";

    let { getUser } = getUserContext();
    let user = getUser();

    const appSettingsApi = getAppSettingsContext();

    onMount(() => {
        if (appSettingsApi?.appSettings?.theme) {
            const theme = appSettingsApi.appSettings.theme as
                | "light"
                | "dark"
                | "system";
            setMode(theme);
        }
    });

    async function handleSave() {
        if (!user.accessToken) return;

        await commands.handleUpdateAppSettings(user.accessToken, {
            theme: userPrefersMode.current || null,
            language: null,
            selectedModel: null,
            modelProxy: null,
            autoLogin: null,
        });

        if (appSettingsApi?.appSettings?.theme) {
            appSettingsApi.appSettings.theme = userPrefersMode.current;
        }
    }
</script>

<Card class="p-6">
    <div class="space-y-4">
        <div>
            <Label class="text-sm font-medium">Theme</Label>
            <div class="my-2 flex items-center gap-2">
                <button onclick={handleSave}>
                    <ThemeSwitch />
                </button>
                <p class="text-sm">
                    {appSettingsApi?.appSettings?.theme}
                </p>
            </div>
        </div>
    </div>
</Card>
