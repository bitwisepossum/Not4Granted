<script lang="ts">
    import { onMount } from "svelte";
    import { getSettings, saveSettings } from "../../components/api";
    import type { Settings } from "../../types";
    import SettingsView from "../../views/SettingsView.svelte";

    let settings = $state<Settings | null>(null);
    let loading = $state(true);
    let saving = $state(false);
    let error = $state<string | null>(null);
    let savedMessage = $state<string | null>(null);

    onMount(async () => {
        try {
            settings = await getSettings();
        } catch (caughtError) {
            error = errorMessage(caughtError);
        } finally {
            loading = false;
        }
    });

    async function handleSave(updatedSettings: Settings): Promise<boolean> {
        saving = true;
        error = null;
        savedMessage = null;

        try {
            settings = await saveSettings(updatedSettings);
            savedMessage = "Settings saved.";

            // TODO: Apply application-wide settings here, or update a shared
            // settings store/context that +layout.svelte owns.
            return true;
        } catch (caughtError) {
            error = errorMessage(caughtError);
            return false;
        } finally {
            saving = false;
        }
    }

    function errorMessage(error: unknown): string {
        return error instanceof Error ? error.message : String(error);
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

    {#if loading}
        <p class="status-message">Loading settings…</p>
    {:else if settings}
        <SettingsView
            {settings}
            {saving}
            {error}
            {savedMessage}
            onsave={handleSave}
        />
    {:else}
        <div class="error-message" role="alert">
            <p>{error ?? "Settings could not be loaded."}</p>
            <!-- TODO: Add a retry button that runs the loading function again. -->
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
        color: var(--text-muted, #666);
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
        border: 1px solid var(--danger-border, #b94a48);
        border-radius: 0.5rem;
        color: var(--danger-text, #8a2424);
    }
</style>