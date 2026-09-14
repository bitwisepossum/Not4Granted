<script lang="ts">
    import { settingsState } from "../../state/settings.svelte";
    import type { Settings } from "../../types";
    import SettingsView from "../../views/SettingsView.svelte";

    let savedMessage = $state<string | null>(null);

    async function handleSave(updatedSettings: Settings): Promise<boolean> {
        savedMessage = null;

        try {
            await settingsState.save(updatedSettings);
            savedMessage = "Settings saved.";
            return true;
        } catch (error) {
            return false; // settingState already sets error
        } finally {
            settingsState.saving = false;
        }
    }

    async function handleRestoreDefaults(): Promise<void> {
        const saved = await handleSave({
            version: 1,
            theme: "system",
            currency: "USD",
            locale: "en-US"
        });

        if (saved) {
            savedMessage = "Settings restored to defaults.";
        }
    }
</script>

<svelte:head>
    <title>Settings | Not4Granted</title>
</svelte:head>

<section class="settings-page">
    <header class="settings-header">
        <div>
            <p class="eyebrow">Application</p>
            <h1>Settings</h1>
        </div>
    </header>

    {#if settingsState.loaded}
        <SettingsView
            settings={settingsState.current}
            saving={settingsState.saving}
            error={settingsState.error}
            {savedMessage}
            onsave={handleSave}
        />
    {:else if settingsState.loading}
        <p class="status-message">Loading settings…</p>
    {:else}
        <div class="error-message" role="alert">
            <p>{settingsState.error ?? "Settings could not be loaded."}</p>
            <p>
                Restore defaults to replace the existing configuration
                and reopen settings.
            </p>

            <button
                class="secondary-button"
                type="button"
                disabled={settingsState.saving}
                onclick={handleRestoreDefaults}
            >
                {settingsState.saving ? "Restoring…" : "Restore default settings"}
            </button>
        </div>
    {/if}
</section>

<style>
    .settings-page {
        width: min(100%, 54rem);
    }

    .settings-header {
        margin-bottom: 1.5rem;
    }

    .settings-header h1,
    .settings-header p {
        margin: 0;
    }

    .eyebrow {
        color: var(--text-muted);
        font-size: 0.78rem;
        font-weight: 700;
        letter-spacing: 0.08em;
        text-transform: uppercase;
    }

    .status-message,
    .error-message {
        padding: 1rem;
    }

    .error-message {
        border: 1px solid var(--danger-border);
        border-radius: 0.5rem;
        color: var(--danger-text);
    }
</style>