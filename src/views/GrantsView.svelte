<script lang="ts">
    import "../styles/app.css";
    import "../styles/data-view.css";
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
        <div class="data-table table-wrapper">
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
                         <tr
                            class="data-interactive"
                            class:expanded={expandedGrantId === grant.id}
                            onclick={() => toggleGrant(grant.id, grant.status)}
                        >
                            <td class="disclosure-cell">
                                <span class="disclosure">
                                    {expandedGrantId === grant.id ? "⌄" : "›"}
                                </span>
                            </td>

                            <td class="data-main">{grant.name}</td>
                            <td class="data-main">{grant.funder}</td>
                            <td class="data-main">{grant.deadline ?? "—"}</td>

                            <td class="data-main">
                                {grant.amountRequested !== undefined
                                    ? `${convertCurrency(grant.amountRequested)} EUR`
                                    : "—"}
                            </td>

                            <td class="data-main">
                                {grant.amountReceived !== undefined
                                    ? `${convertCurrency(grant.amountReceived)} EUR`
                                    : "—"}
                            </td>

                            <td class="data-main">
                                <span class="status">{grant.status}</span>
                            </td>
                        </tr>

                        {#if expandedGrantId === grant.id}
                            <tr class="expanded-row">
                                <td colspan="7">
                                    <div class="data-details">
                                        <!-- TODO: Add status selection -->
                                        <!--<label>
                                            <span>Status</span>

                                            <select bind:value={selectedStatus}>
                                                <option value="Planning">Planning</option>
                                                <option value="Submitted">Submitted</option>
                                                <option value="Accepted">Accepted</option>
                                                <option value="Rejected">Rejected</option>
                                            </select>
                                        </label>-->

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

.disclosure-column,
.disclosure-cell {
    width: 2rem;
    padding-right: 0;
}

.expanded-row td {
    padding: 0;
}
</style>