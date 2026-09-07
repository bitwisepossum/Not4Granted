<script lang="ts">
    import "../app.css";
    import { convertCurrency } from "../components/format";
    import type { Grant, GrantStatus } from "../types";

    let {
        grants
    }: {
        grants: Grant[];
    } = $props();

    let expandedGrantId = $state<number | undefined>(undefined);
    let selectedStatus = $state<GrantStatus>("Planning");

    function toggleGrant(id: number, status: GrantStatus) {
        if (expandedGrantId === id) {
            expandedGrantId = undefined;
        } else {
            expandedGrantId = id;
            selectedStatus = status;
        }
    }
</script>

<section class="view">
    <header class="page-header">
        <div>
            <h1>Grants</h1>
            <p>Grant applications and funding decisions</p>
        </div>

        <a class="button-link" href="/grants/new">Add grant</a>
    </header>

    {#if grants.length === 0}
        <div class="empty-state">
            <h2>No grants</h2>
            <p>Add the first grant application to start tracking it.</p>
        </div>
    {:else}
        <div class="table-wrapper">
            <table>
                <thead>
                    <tr>
                        <th class="disclosure-column"></th>
                        <th>Name</th>
                        <th>Funder</th>
                        <th>Deadline</th>
                        <th>Requested</th>
                        <th>Received</th>
                        <th>Status</th>
                    </tr>
                </thead>

                <tbody>
                    {#each grants as grant}
                        <tr class:expanded={expandedGrantId === grant.id} onclick={() => toggleGrant(grant.id, grant.status)}>
                            <td class="disclosure-cell">
                                <span class="disclosure">
                                    {expandedGrantId === grant.id ? "⌄" : "›"}
                                </span>
                            </td>

                            <td class="main-cell">{grant.name}</td>
                            <td>{grant.funder}</td>
                            <td>{grant.deadline ?? "—"}</td>

                            <td>
                                {grant.amountRequested !== undefined
                                    ? `${convertCurrency(grant.amountRequested)} EUR`
                                    : "—"}
                            </td>

                            <td>
                                {grant.amountReceived !== undefined
                                    ? `${convertCurrency(grant.amountReceived)} EUR`
                                    : "—"}
                            </td>

                            <td>
                                <span class="status">{grant.status}</span>
                            </td>
                        </tr>

                        {#if expandedGrantId === grant.id}
                            <tr class="expanded-row">
                                <td colspan="7">
                                    <div class="quick-actions">
                                        <label>
                                            <span>Status</span>

                                            <select bind:value={selectedStatus}>
                                                <option value="Planning">Planning</option>
                                                <option value="Submitted">Submitted</option>
                                                <option value="Accepted">Accepted</option>
                                                <option value="Rejected">Rejected</option>
                                            </select>
                                        </label>

                                        <a
                                            class="button-link"
                                            href={`/grants/${grant.id}`}
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
        border: 1px solid #333;
        border-radius: 0.6rem;
    }

    table {
        width: 100%;
        border-collapse: collapse;
    }

    th,
    td {
        padding: 0.85rem 1rem;
        text-align: left;
        border-bottom: 1px solid #333;
    }

    .main-cell {
        font-weight: 600;
    }

    tbody > tr:not(.expanded-row) {
    cursor: pointer;
}

tbody > tr:not(.expanded-row):hover {
    background: rgba(255, 255, 255, 0.035);
}

tbody > tr.expanded {
    background: rgba(255, 255, 255, 0.05);
}

.disclosure-column,
.disclosure-cell {
    width: 2rem;
    padding-right: 0;
}

.disclosure {
    display: inline-block;
    width: 1rem;

    color: #aaa;
    font-size: 1.1rem;
    line-height: 1;

    transition: color 120ms ease;
}

tbody > tr:hover .disclosure {
    color: #060606;
    font-weight: 800;
}

tbody > tr.expanded .disclosure {
    color: #060606;
    font-weight: 800;
}

.expanded-row td {
    padding: 0;
}

.quick-actions {
    display: flex;
    align-items: end;
    gap: 1rem;

    padding: 1rem 1.25rem 1rem 3rem;

    background: rgba(255, 255, 255, 0.02);
    border-bottom: 1px solid #333;
}

.quick-actions label {
    display: grid;
    gap: 0.35rem;
}

.quick-actions label > span {
    color: #aaa;
    font-size: 0.8rem;
    font-weight: 600;
}
</style>