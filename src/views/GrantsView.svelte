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
                        <th>Name</th>
                        <th>Funder</th>
                        <th>Deadline</th>
                        <th>Amount requested</th>
                        <th>Amount received</th>
                        <th>Status</th>
                    </tr>
                </thead>

                <tbody>
                    {#each grants as grant}
                        <tr onclick={() => toggleGrant(grant.id, grant.status)}>
                            <td class="main-cell">{grant.name}</td>
                            <td>{grant.funder}</td>
                            <td>{grant.deadline}</td>
                            <td>{grant.amountRequested !== undefined ? convertCurrency(grant.amountRequested) : "-"} EUR</td>
                            <td>{grant.amountReceived !== undefined ? convertCurrency(grant.amountReceived) : "-"} EUR</td>
                            <td>{grant.status}</td>
                        </tr>

                        {#if expandedGrantId === grant.id}
                            {#key grant.id}
                                <tr class="expanded-row">
                                    <td colspan="6">
                                        <select bind:value={selectedStatus}>
                                            <option value="Planning">Planning</option>
                                            <option value="Submitted">Submitted</option>
                                            <option value="Accepted">Accepted</option>
                                            <option value="Rejected">Rejected</option>
                                        </select>
                                    </td>
                                </tr>
                            {/key}
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
</style>