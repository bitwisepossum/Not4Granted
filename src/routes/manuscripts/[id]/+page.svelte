<script lang="ts">
    import { page } from "$app/state";
    import "../../../styles/item.css";

    import type { Manuscript, ManuscriptStatus } from "../../../types";

    let manuscript = $state<Manuscript | undefined>(undefined);
    let draft = $state<Manuscript | undefined>(undefined);
    let isEditing = $state(false);
    let isLoading = $state(true);
    let error = $state<string | undefined>(undefined);

    /*
     * TODO: ROUTE + LOAD LOGIC
     */

    const routeId = $derived(page.params.id);

    function beginEdit() {
        if (!manuscript) return;

        draft = structuredClone(manuscript);
        isEditing = true;
    }

    function cancelEdit() {
        if (!manuscript) return;

        draft = structuredClone(manuscript);
        isEditing = false;
    }

    async function saveManuscript() {
        if (!draft) return;

        /*
         * TODO: SAVE LOGIC
         */
    }

    async function deleteManuscript() {
        /*
         * TODO: DELETE LOGIC
         */
    }

    function display(value: string | number | undefined) {
        return value === undefined || value === "" ? "—" : String(value);
    }
</script>

<svelte:head>
    <title>{manuscript?.shortName ?? manuscript?.title ?? `Manuscript ${routeId}`} | Not4Granted</title>
</svelte:head>

<section class="item-page">
    <nav class="item-breadcrumbs" aria-label="Breadcrumb">
        <a href="/">Dashboard</a>
        <span aria-hidden="true">/</span>
        <a href="/manuscripts">Manuscripts</a>
        <span aria-hidden="true">/</span>
        <span class="item-breadcrumb-current">
            {manuscript?.shortName ?? manuscript?.title ?? `Manuscript ${routeId}`}
        </span>
    </nav>

    {#if isLoading}
        <div class="item-placeholder">
            <p class="item-dev-note">
                TODO: load Manuscript #{routeId} from the Tauri backend using page.params.id.
            </p>
        </div>
    {:else if error}
        <div class="item-placeholder">
            <strong>Could not load manuscript</strong>
            <span>{error}</span>
        </div>
    {:else if !manuscript || !draft}
        <div class="item-placeholder">
            <strong>Manuscript not found</strong>
            <span>No manuscript data is available for this route.</span>
        </div>
    {:else}
        <header class="item-toolbar">
            <div class="item-heading">
                <h1>{manuscript.title}</h1>
                <h2>{manuscript.journal ?? "No journal selected"}</h2>
            </div>

            <div class="item-actions">
                {#if isEditing}
                    <button class="item-action" type="button" onclick={cancelEdit}>
                        Cancel
                    </button>
                    <button class="item-action primary" type="button" onclick={saveManuscript}>
                        Save
                    </button>
                {:else}
                    <button class="item-action primary" type="button" onclick={beginEdit}>
                        Edit
                    </button>
                    <button class="item-action danger" type="button" onclick={deleteManuscript}>
                        Delete
                    </button>
                {/if}
            </div>
        </header>

        <div class="item-sheet">
            <section class="item-section">
                <header class="item-section-header">
                    <h2>Overview</h2>
                </header>

                <div class="item-grid">
                    <div class="item-field full">
                        <span class="item-label">Title</span>
                        {#if isEditing}
                            <input class="item-input" bind:value={draft.title} />
                        {:else}
                            <div class="item-value">{display(manuscript.title)}</div>
                        {/if}
                    </div>

                    <div class="item-field">
                        <span class="item-label">Short name</span>
                        {#if isEditing}
                            <input class="item-input" bind:value={draft.shortName} />
                        {:else}
                            <div class="item-value">{display(manuscript.shortName)}</div>
                        {/if}
                    </div>

                    <div class="item-field">
                        <span class="item-label">Status</span>
                        {#if isEditing}
                            <select class="item-select" bind:value={draft.status}>
                                {#each statuses as status}
                                    <option value={status}>{status}</option>
                                {/each}
                            </select>
                        {:else}
                            <span class="item-status">{manuscript.status}</span>
                        {/if}
                    </div>

                    <div class="item-field full">
                        <span class="item-label">Journal</span>
                        {#if isEditing}
                            <input class="item-input" bind:value={draft.journal} />
                        {:else}
                            <div class="item-value">{display(manuscript.journal)}</div>
                        {/if}
                    </div>
                </div>
            </section>

            <section class="item-section">
                <header class="item-section-header">
                    <h2>Workflow</h2>
                </header>

                <div class="item-grid">
                    <div class="item-field full">
                        <span class="item-label">Next action</span>
                        {#if isEditing}
                            <input class="item-input" bind:value={draft.nextAction} />
                        {:else}
                            <div class="item-value">{display(manuscript.nextAction)}</div>
                        {/if}
                    </div>

                    <div class="item-field full">
                        <span class="item-label">DOI</span>
                        {#if isEditing}
                            <input class="item-input" bind:value={draft.doi} />
                        {:else}
                            <div class="item-value">{display(manuscript.doi)}</div>
                        {/if}
                    </div>
                </div>
            </section>

            <section class="item-section">
                <header class="item-section-header">
                    <h2>Timeline</h2>
                </header>

                <div class="item-grid">
                    <div class="item-field">
                        <span class="item-label">Submitted</span>
                        {#if isEditing}
                            <input class="item-input" type="date" bind:value={draft.submittedAt} />
                        {:else}
                            <div class="item-value">{display(manuscript.submittedAt)}</div>
                        {/if}
                    </div>

                    <div class="item-field">
                        <span class="item-label">Decision</span>
                        {#if isEditing}
                            <input class="item-input" type="date" bind:value={draft.decisionAt} />
                        {:else}
                            <div class="item-value">{display(manuscript.decisionAt)}</div>
                        {/if}
                    </div>

                    <div class="item-field">
                        <span class="item-label">Published</span>
                        {#if isEditing}
                            <input class="item-input" type="date" bind:value={draft.publishedAt} />
                        {:else}
                            <div class="item-value">{display(manuscript.publishedAt)}</div>
                        {/if}
                    </div>
                </div>
            </section>

            <section class="item-section">
                <header class="item-section-header">
                    <h2>Notes</h2>
                </header>

                <div class="item-grid">
                    <div class="item-field full">
                        {#if isEditing}
                            <textarea class="item-textarea" bind:value={draft.notes}></textarea>
                        {:else}
                            <div class="item-value" class:muted={!manuscript.notes}>
                                {display(manuscript.notes)}
                            </div>
                        {/if}
                    </div>
                </div>
            </section>
        </div>
    {/if}
</section>