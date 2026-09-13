<script lang="ts">
    import "../styles/app.css";
    import "../styles/data-view.css";
    import type { Manuscript, ManuscriptStatus } from "../types";
    import { formatDate } from "../components/format";
    import { settingsState } from "../state/settings.svelte";

    let {
        manuscripts
    }: {
        manuscripts: Manuscript[];
    } = $props();

    let expandedManuscriptId = $state<number | undefined>(undefined);
    let selectedStatus = $state<ManuscriptStatus>("Idea");

    function toggleManuscript(
        id: number,
        status: ManuscriptStatus
    ) {
        if (expandedManuscriptId === id) {
            expandedManuscriptId = undefined;
        } else {
            expandedManuscriptId = id;
            selectedStatus = status;
        }
    }

    function statusClass(status: string): string {
        return status.toLowerCase();
    }
</script>

<section class="view">
    <header class="page-header">
        <div>
            <h1>Manuscripts</h1>
            <p>Publication progress and next actions</p>
        </div>

        <a class="primary" href="/manuscripts/new">
            Add manuscript
        </a>
    </header>

    {#if manuscripts.length === 0}
        <div class="empty-state">
            <h2>No manuscripts</h2>
            <p>Add the first manuscript to start tracking it.</p>
        </div>
    {:else}
        <div class="table-wrapper">
            <table class="data-table">
                <thead>
                    <tr>
                        <th>Manuscript</th>
                        <th>Next action</th>
                        <th>Submitted</th>
                        <th>Decision</th>
                        <th>Status</th>
                    </tr>
                </thead>

                <tbody>
                    {#each manuscripts as manuscript}
                        <tr
                            class="data-row"
                            class:expanded={
                                expandedManuscriptId === manuscript.id
                            }
                            onclick={() =>
                                toggleManuscript(
                                    manuscript.id,
                                    manuscript.status
                                )}
                        >
                            <td>
                                <div class="manuscript-main">
                                    <strong>
                                        {manuscript.title}
                                    </strong>

                                    <span>
                                        {manuscript.journal ??
                                            "No journal selected"}
                                    </span>
                                </div>
                            </td>

                            <td>
                                {manuscript.nextAction ?? "—"}
                            </td>

                            <td>
                                {formatDate(manuscript.submittedAt, settingsState.current.locale) ?? "—"}
                            </td>

                            <td>
                                {formatDate(manuscript.decisionAt, settingsState.current.locale) ?? "—"}
                            </td>

                            <td>
                                <span
                                    class={`status ${statusClass(
                                        manuscript.status
                                    )}`}
                                >
                                    {manuscript.status}
                                </span>
                            </td>
                        </tr>

                        {#if expandedManuscriptId === manuscript.id}
                            <tr class="expanded-row">
                                <td colspan="5">
                                    <div class="quick-actions">
                                        <!--TODO <div class="quick-status">
                                            <label
                                                for={`status-${manuscript.id}`}
                                            >
                                                Status
                                            </label>

                                            <select
                                                id={`status-${manuscript.id}`}
                                                value={selectedStatus}
                                                onchange={(event) =>
                                                    handleStatusChange(
                                                        manuscript.id,
                                                        event
                                                    )}
                                                onclick={(event) =>
                                                    event.stopPropagation()}
                                            >
                                                <option value="Idea">
                                                    Idea
                                                </option>
                                                <option value="Drafting">
                                                    Drafting
                                                </option>
                                                <option value="Submitted">
                                                    Submitted
                                                </option>
                                                <option value="Revision">
                                                    Revision
                                                </option>
                                                <option value="Accepted">
                                                    Accepted
                                                </option>
                                                <option value="Published">
                                                    Published
                                                </option>
                                                <option value="Rejected">
                                                    Rejected
                                                </option>
                                            </select>
                                        </div>-->

                                        <a
                                            class="button-link"
                                            href={`/manuscripts/${manuscript.id}`}
                                        >
                                            View details
                                        </a>
                                    </div>
                                </td>
                            </tr>
                        {/if}
                    {/each}
                </tbody>
            </table>
        </div>
    {/if}
</section>

<style>
    .table-wrapper {
        overflow-x: auto;
    }

    .data-table {
        width: 100%;
        border-collapse: collapse;
    }

    .data-table th,
    .data-table td {
        padding: 0.9rem 1rem;
        text-align: left;
        vertical-align: middle;
    }

    .data-table th {
        font-size: 0.8rem;
        font-weight: 600;
        color: var(--text-subtle);
        border-bottom: 1px solid var(--border);
    }

    .data-row {
        border-bottom: 1px solid var(--border-subtle);
        cursor: pointer;
    }

    .data-row:hover,
    .data-row.expanded {
        background: var(--surface-hover);
    }

    .manuscript-main {
        display: grid;
        gap: 0.2rem;
    }

    .manuscript-main strong {
        font-weight: 600;
    }

    .manuscript-main span {
        font-size: 0.85rem;
        color: var(--text-muted);
    }

    .data-table th:first-child,
    .data-table td:first-child {
        width: 40%;
    }

    .data-table th:last-child,
    .data-table td:last-child {
        width: 1%;
        white-space: nowrap;
    }
</style>