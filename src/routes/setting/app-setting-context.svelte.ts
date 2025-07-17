import { getContext, setContext } from "svelte";
import type { AppSettings } from "@/tauri";

export const settingKey = Symbol("app-setting");

export type AppSettingsText = {
    appSettings: AppSettings | undefined;
};
export function createAppSettingsContext(): AppSettingsText {
    let appSettings: AppSettings | undefined = $state(undefined);

    return {
        get appSettings() {
            return appSettings;
        },
        set appSettings(as: AppSettings | undefined) {
            appSettings = as;
        },
    };
}

export const getAppSettingsContext = () => {
    return getContext(settingKey) as AppSettingsText;
};

export const setAppSettingsContext = (context: AppSettingsText) => {
    setContext(settingKey, context);
};
