<script lang="ts">
    import "../styles/app.css";
    import "../styles/data-view.css";
    import type { Manuscript, ManuscriptStatus } from "../types";

    let {
        manuscripts
    }: {
        manuscripts: Manuscript[];
    } = $props();

    let expandedManuscriptId = $state<number | undefined>(undefined);
    let selectedStatus = $state<ManuscriptStatus>("Idea");

    function toggleManuscript(id: number, status: ManuscriptStatus) {
        if (expandedManuscriptId === id) {
            expandedManuscriptId = undefined;
        } else {
            expandedManuscriptId = id;
        }
        selectedStatus = status;
    }

</script>

<section class="view">
    <header class="page-header">
        <div>
            <h1>Manuscripts</h1>
            <p>Publication progress and next actions</p>
        </div>

        <a class="primary" href="/manuscripts/new">Add manuscript</a>
    </header>

    {#if manuscripts.length === 0}
        <div class="empty-state">
            <h2>No manuscripts</h2>
            <p>Add the first manuscript to start tracking it.</p>
        </div>
    {:else}
        <div class="data-list">
            {#each manuscripts as manuscript}
                <article class="item">
                    <button
                        class="item-summary data-interactive"
                        class:expanded={expandedManuscriptId === manuscript.id}
                        onclick={() =>
                            toggleManuscript(
                                manuscript.id,
                                manuscript.status
                            )}
                    >
                        <span class="disclosure">
                            {expandedManuscriptId === manuscript.id ? "⌄" : "›"}
                        </span>

                        <div>
                            <strong class="data-main">
                                {manuscript.title}
                            </strong>

                            <span class="data-secondary">
                                {manuscript.journal ?? "No journal selected"}
                            </span>
                        </div>

                        <span class="status">
                            {manuscript.status}
                        </span>
                    </button>

                    {#if expandedManuscriptId === manuscript.id}
                        <div class="data-details">
                            <div>
                                <span>Next action</span>
                                <strong>
                                    {manuscript.nextAction ?? "None"}
                                </strong>
                            </div>

                            <a
                                class="button-link"
                                href={`/manuscripts/${manuscript.id}`}
                            >
                                View details
                            </a>
                        </div>
                    {/if}
                </article>
            {/each}
        </div>
    {/if}
</section>

<style>
.item + .item {
    border-top: 1px solid #333;
}

.item-summary {
    display: grid;
    grid-template-columns: 1.5rem 1fr auto;
    align-items: center;
    gap: 0.75rem;

    width: 100%;
    padding: 0.9rem 1rem;

    border: 0;
    background: transparent;
    color: inherit;

    text-align: left;
}

.item-summary > div {
    display: grid;
    gap: 0.2rem;
}
</style>