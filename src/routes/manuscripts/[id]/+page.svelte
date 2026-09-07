<script lang="ts">
    import { page } from "$app/state";
    import { goto } from "$app/navigation";
    import "../../../styles/item.css";
    import type { Manuscript } from "../../../types";
    import { manuscriptStatuses } from "../../../types";
    import { onMount } from "svelte";
    import { getManuscriptById, updateManuscript, deleteManuscript } from "../../../components/api";


    let manuscript = $state<Manuscript | undefined>(undefined);
    let draft = $state<Manuscript | undefined>(undefined);
    let isEditing = $state(false);
    let isLoading = $state(true);
    let error = $state<string | undefined>(undefined);
    let showDeleteConfirm = $state(false);
    let isDeleting = $state(false);

    const routeId = $derived(page.params.id);

    onMount(async () => {
        try {
            const id = Number(routeId);
            if (!Number.isInteger(id)) {
                throw new Error("Invalid manuscript ID");
            }

            manuscript = await getManuscriptById(id);
            draft = $state.snapshot(manuscript);
            console.log("Fetched manuscript:", manuscript);
        } catch (err) {
            error = `Failed to load manuscript ${routeId}: ${err}`;
        } finally {
            isLoading = false;
        }
    })

    function beginEdit() {
        if (!manuscript) return;

        draft = $state.snapshot(manuscript);
        isEditing = true;
    }

    function cancelEdit() {
        if (!manuscript) return;

        draft = $state.snapshot(manuscript);
        isEditing = false;
    }

    async function handleSave() {
        if (!draft) return;

        const id = Number(routeId);
        if (!Number.isInteger(id)) {
            error = "Invalid manuscript ID";
            return;
        }

        await updateManuscript(draft);
        manuscript = await getManuscriptById(id);
        isEditing = false;
    }

    async function handleDelete() {
        if (!manuscript) return;

        const id = Number(routeId);
        if (!Number.isInteger(id)) {
            error = "Invalid manuscript ID";
            return;
        }

        if (!await getManuscriptById(id)) {
            error = "Manuscript not found";
            return;
        } else {
            await deleteManuscript(id);
            manuscript = undefined;
            draft = undefined;
            isEditing = false;
            isDeleting = false;
            showDeleteConfirm = false;
            await goto("/manuscripts");
        }
    }

    function display(value: string | number | undefined) {
        return value === undefined || value === "" ? "—" : String(value);
    }
</script>

<svelte:head>
    <title>{manuscript?.shortName ?? manuscript?.title ?? `Manuscript ${routeId}`} | Not4Granted</title>
</svelte:head>

<section class="item-page">
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
                    <button class="item-action primary" type="button" onclick={handleSave}>
                        Save
                    </button>
                {:else}
                    <button class="item-action primary" type="button" onclick={beginEdit}>
                        Edit
                    </button>
                    <button class="item-action danger" type="button" onclick={() => showDeleteConfirm = true}>
                        Delete
                    </button>
                    {#if showDeleteConfirm}
                        <div class="confirm-backdrop">
                            <div
                                class="confirm-dialog"
                                role="dialog"
                                aria-modal="true"
                                aria-labelledby="delete-title"
                            >
                                <h2 id="delete-title">Delete grant?</h2>

                                <p>
                                    This action cannot be undone.
                                </p>

                                <div class="confirm-actions">
                                    <button
                                        type="button"
                                        onclick={() => showDeleteConfirm = false}
                                        disabled={isDeleting}
                                    >
                                        Cancel
                                    </button>

                                    <button
                                        type="button"
                                        class="danger"
                                        onclick={handleDelete}
                                        disabled={isDeleting}
                                    >
                                        {isDeleting ? "Deleting…" : "Delete"}
                                    </button>
                                </div>
                            </div>
                        </div>
                    {/if}
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
                                {#each manuscriptStatuses as status}
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
                            <input 
                            class="item-input" 
                                type="date" 
                                bind:value={draft.submittedAt} 
                                onchange={(event) => {
                                    const input = event.currentTarget as HTMLInputElement;
                                    setTimeout(() => input.blur(), 0);
                                }} 
                            />
                        {:else}
                            <div class="item-value">{display(manuscript.submittedAt)}</div>
                        {/if}
                    </div>

                    <div class="item-field">
                        <span class="item-label">Decision</span>
                        {#if isEditing}
                            <input class="item-input" 
                                type="date" 
                                bind:value={draft.decisionAt} 
                                onchange={(event) => {
                                    const input = event.currentTarget as HTMLInputElement;
                                    setTimeout(() => input.blur(), 0);
                                }} 
                            />
                        {:else}
                            <div class="item-value">{display(manuscript.decisionAt)}</div>
                        {/if}
                    </div>

                    <div class="item-field">
                        <span class="item-label">Published</span>
                        {#if isEditing}
                            <input class="item-input" 
                                type="date" 
                                bind:value={draft.publishedAt} 
                                onchange={(event) => {
                                    const input = event.currentTarget as HTMLInputElement;
                                    setTimeout(() => input.blur(), 0);
                                }} 
                            />
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