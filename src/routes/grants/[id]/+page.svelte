<script lang="ts">
    import { page } from "$app/state";
    import "../../../styles/item.css";
    import type { Grant, GrantStatus } from "../../../types";
    import { onMount } from "svelte";
    import { getGrantById, updateGrant } from "../../../components/api";

    const statuses: GrantStatus[] = [
        "Planning",
        "Submitted",
        "Accepted",
        "Rejected"
    ];

    let grant = $state<Grant | undefined>(undefined);
    let draft = $state<Grant | undefined>(undefined);
    let isEditing = $state(false);
    let isLoading = $state(true);
    let error = $state<string | undefined>(undefined);

    let routeId = $derived(page.params.id);

    onMount(async () => {
        try {
            const id = Number(routeId);
            if (!Number.isInteger(id)) {
                throw new Error("Invalid grant ID");
            }

            grant = await getGrantById(id);
            draft = $state.snapshot(grant);
            console.log("Fetched grant:", grant);
        } catch (err) {
            error = `Failed to load grant ${routeId}: ${err}`;
        } finally {
            isLoading = false;
        }
    })

    console.log("Grant ID from route:", routeId);
    console.log("Loaded grant:", grant);
    console.log("Draft grant:", draft);

    function beginEdit() {
        if (!grant) return;

        draft = $state.snapshot(grant);
        isEditing = true;
    }

    function cancelEdit() {
        if (!grant) return;

        draft = $state.snapshot(grant);
        isEditing = false;
    }

    async function saveGrant() {
        if (!draft) return;

        const id = Number(routeId);
        if (!Number.isInteger(id)) {
            throw new Error("Invalid grant ID");
        }

        await updateGrant(draft);
        grant = await getGrantById(id);
        isEditing = false;
    }

    async function deleteGrant() {
        /*
         * TODO: DELETE LOGIC
         */
    }

    function display(value: string | number | undefined) {
        return value === undefined || value === "" ? "—" : String(value);
    }
</script>

<svelte:head>
    <title>{grant?.name ?? `Grant ${routeId}`} | Not4Granted</title>
</svelte:head>

<section class="item-page">
    {#if isLoading}
        <div class="item-placeholder">
            <p class="item-dev-note">
                TODO: load Grant #{routeId} from the Tauri backend using page.params.id.
            </p>
        </div>
    {:else if error}
        <div class="item-placeholder">
            <strong>Could not load grant</strong>
            <span>{error}</span>
        </div>
    {:else if !grant || !draft}
        <div class="item-placeholder">
            <strong>Grant not found</strong>
            <span>No grant data is available for this route.</span>
        </div>
    {:else}
        <header class="item-toolbar">
            <div class="item-heading">
                <h1>{grant.name}</h1>
                <h2>{grant.funder}</h2>
            </div>

            <div class="item-actions">
                {#if isEditing}
                    <button class="item-action" type="button" onclick={cancelEdit}>
                        Cancel
                    </button>
                    <button class="item-action primary" type="button" onclick={saveGrant}>
                        Save
                    </button>
                {:else}
                    <button class="item-action primary" type="button" onclick={beginEdit}>
                        Edit
                    </button>
                    <button class="item-action danger" type="button" onclick={deleteGrant}>
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
                    <div class="item-field">
                        <span class="item-label">Name</span>
                        {#if isEditing}
                            <input class="item-input" bind:value={draft.name} />
                        {:else}
                            <div class="item-value">{display(grant.name)}</div>
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
                            <span class="item-status">{grant.status}</span>
                        {/if}
                    </div>

                    <div class="item-field">
                        <span class="item-label">Funder</span>
                        {#if isEditing}
                            <input class="item-input" bind:value={draft.funder} />
                        {:else}
                            <div class="item-value">{display(grant.funder)}</div>
                        {/if}
                    </div>

                    <div class="item-field">
                        <span class="item-label">Call</span>
                        {#if isEditing}
                            <input class="item-input" bind:value={draft.callName} />
                        {:else}
                            <div class="item-value">{display(grant.callName)}</div>
                        {/if}
                    </div>
                </div>
            </section>

            <section class="item-section">
                <header class="item-section-header">
                    <h2>Funding</h2>
                </header>

                <div class="item-grid">
                    <div class="item-field">
                        <span class="item-label">Requested</span>
                        {#if isEditing}
                            <input class="item-input" type="number" bind:value={draft.amountRequested} />
                        {:else}
                            <div class="item-value">{display(grant.amountRequested)}</div>
                        {/if}
                    </div>

                    <div class="item-field">
                        <span class="item-label">Received</span>
                        {#if isEditing}
                            <input class="item-input" type="number" bind:value={draft.amountReceived} />
                        {:else}
                            <div class="item-value">{display(grant.amountReceived)}</div>
                        {/if}
                    </div>

                    <div class="item-field">
                        <span class="item-label">Currency</span>
                        {#if isEditing}
                            <input class="item-input" bind:value={draft.currency} />
                        {:else}
                            <div class="item-value">{display(grant.currency)}</div>
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
                        <span class="item-label">Deadline</span>
                        {#if isEditing}
                            <input class="item-input" 
                                type="date" 
                                bind:value={draft.deadline}
                                onchange={(event) => {
                                    const input = event.currentTarget as HTMLInputElement;
                                    setTimeout(() => input.blur(), 0);
                                }} 
                            />
                        {:else}
                            <div class="item-value">{display(grant.deadline)}</div>
                        {/if}
                    </div>

                    <div class="item-field">
                        <span class="item-label">Submitted</span>
                        {#if isEditing}
                            <input class="item-input" type="date" 
                                bind:value={draft.submittedAt} onchange={(event) => {
                                const input = event.currentTarget as HTMLInputElement;
                                setTimeout(() => input.blur(), 0);
                                }} 
                            />
                        {:else}
                            <div class="item-value">{display(grant.submittedAt)}</div>
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
                            <div class="item-value">{display(grant.decisionAt)}</div>
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
                            <div class="item-value" class:muted={!grant.notes}>
                                {display(grant.notes)}
                            </div>
                        {/if}
                    </div>
                </div>
            </section>
        </div>
    {/if}
</section>