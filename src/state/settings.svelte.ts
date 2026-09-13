import { getSettings, saveSettings } from "../components/api";
import type { Settings } from "../types";

const defaults: Settings = {
    locale: "en-US",
    theme: "system",
    version: 1,
    currency: "USD"
};

class SettingsState {
    current = $state<Settings>({ ...defaults });

    loaded = $state(false);
    loading = $state(false);
    saving = $state(false);
    error = $state<string | null>(null);

    async load(): Promise<void> {
        if (this.loaded || this.loading) return;

        this.loading = true;
        this.error = null;

        try {
            this.current = await getSettings();
            this.loaded = true;
        } catch (error) {
            this.error = error instanceof Error
                ? error.message
                : String(error);
        } finally {
            this.loading = false;
        }
    }

    async save(settings: Settings): Promise<void> {
        this.saving = true;
        this.error = null;
        
        try {
            this.current = await saveSettings(settings);
        } catch (error) {
            this.error = error instanceof Error
                ? error.message
                : String(error);

                return Promise.reject(error);
        } finally {
            this.saving = false;
        }
    }
}

export const settingsState = new SettingsState();