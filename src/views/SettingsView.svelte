<script lang="ts">
    import type { Locale, Settings, Theme } from "../types";
    import { LocaleNames, ThemeName } from "../types";

    type Props = {
        settings: Settings;
        saving?: boolean;
        error?: string | null;
        savedMessage?: string | null;
        onsave: (settings: Settings) => Promise<boolean>;
    };

    let {
        settings,
        saving = false,
        error = null,
        savedMessage = null,
        onsave
    }: Props = $props();

    const locales = Object.entries(LocaleNames).map(([value, label]) => ({
        value,
        label
    }));

    const themes = Object.entries(ThemeName).map(([value, label]) => ({
        value: value as Theme,
        label
    }));

    let draft = $state<Settings>({ ...settings });

    let changed = $derived(
        draft.locale !== settings.locale ||
            draft.theme !== settings.theme ||
            draft.currency !== settings.currency
    );

    function resetForm(): void {
        draft = { ...settings };
    }

    async function submit(event: SubmitEvent): Promise<void> {
        event.preventDefault();

        const normalizedSettings: Settings = {
            ...draft,
            currency: draft.currency.trim().toUpperCase()
        };

        // Copy the rune-backed object before it crosses the Tauri IPC boundary.
        const saved = await onsave({ ...normalizedSettings });

        if (saved) {
            draft = { ...normalizedSettings };
        }
    }
</script>

<form class="settings-form" onsubmit={submit}>
    <section class="settings-card" aria-labelledby="appearance-heading">
        <div class="section-heading">
            <div>
                <h2 id="appearance-heading">Appearance</h2>
                <p>Choose how Not4Granted should look.</p>
            </div>
        </div>

        <fieldset>
            <legend>Theme</legend>

            <div class="radio-options">
                <select id="settings-theme" bind:value={draft.theme}>
                    {#each themes as theme}
                        <option value={theme.value}>{theme.label}</option>
                    {/each}
                </select>
            </div>

        </fieldset>
    </section>

    <section class="settings-card" aria-labelledby="regional-heading">
        <div class="section-heading">
            <div>
                <h2 id="regional-heading">Language and locale</h2>
                <p>Control date and time locale and default monetary formatting.</p>
            </div>
        </div>

        <div class="field-grid">
            <label class="field">
                <span>Locale</span>
                <select bind:value={draft.locale}>
                    {#each locales as locale}
                        <option value={locale.value}>{locale.label}</option>
                    {/each}
                </select>
            </label>

            <label class="field">
                <span>Currency</span>
                <input
                    bind:value={draft.currency}
                    maxlength="3"
                    autocomplete="off"
                    spellcheck="false"
                    placeholder="EUR"
                />
                <small>Three-letter ISO 4217 currency code, for example EUR, SEK, JPY or USD.</small>
            </label>
        </div>

    </section>

    {#if error}
        <p class="feedback error" role="alert">{error}</p>
    {:else if savedMessage}
        <p class="feedback success" role="status">{savedMessage}</p>
    {/if}

    <footer class="form-actions">
        <button
            class="secondary-button"
            type="button"
            disabled={!changed || saving}
            onclick={resetForm}
        >
            Reset
        </button>

        <button
            class="primary-button"
            type="submit"
            disabled={!changed || saving}
        >
            {saving ? "Saving…" : "Save settings"}
        </button>
    </footer>
</form>

<style>
    .settings-form {
        display: grid;
        gap: 1rem;
    }

    .settings-card {
        background: var(--surface);
        border: 1px solid var(--border);
        border-radius: 0.75rem;
        padding: 1.25rem;
    }

    .section-heading {
        margin-bottom: 1.25rem;
    }

    .section-heading h2,
    .section-heading p {
        margin: 0;
    }

    .section-heading p,
    small {
        color: var(--text-muted);
    }

    fieldset {
        border: 0;
        margin: 0;
        padding: 0;
    }

    legend,
    .field > span {
        font-weight: 650;
        margin-bottom: 0.5rem;
    }

    .radio-options {
        display: flex;
        flex-wrap: wrap;
        gap: 0.75rem 1.25rem;
    }

    .field-grid {
        display: grid;
        gap: 1rem;
        grid-template-columns: repeat(auto-fit, minmax(14rem, 1fr));
    }

    .field {
        display: flex;
        flex-direction: column;
    }

    select,
    input {
        background: var(--surface);
        border: 1px solid var(--border);
        border-radius: 0.4rem;
        color: inherit;
        font: inherit;
        padding: 0.65rem 0.75rem;
    }

    small {
        margin-top: 0.35rem;
    }

    .feedback {
        border-radius: 0.5rem;
        margin: 0;
        padding: 0.75rem 1rem;
    }

    .feedback.error {
        background: var(--danger-bg);
        color: var(--danger-text);
    }

    .feedback.success {
        background: var(--success-bg);
        color: var(--success-text);
    }

    .form-actions {
        display: flex;
        justify-content: flex-end;
        gap: 0.75rem;
        padding-top: 0.25rem;
    }

    button {
        border-radius: 0.45rem;
        cursor: pointer;
        font: inherit;
        font-weight: 650;
        padding: 0.65rem 1rem;
    }

    button:disabled {
        cursor: not-allowed;
        opacity: 0.55;
    }

    .secondary-button {
        background: transparent;
        border: 1px solid var(--border);
        color: inherit;
    }

    .primary-button {
        background: var(--accent);
        border: 1px solid transparent;
        color: var(--accent-text);
    }
</style>