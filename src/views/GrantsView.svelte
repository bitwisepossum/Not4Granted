<script lang="ts">
    import type { Manuscript } from "../types";

    let {
        manuscripts,
        onNavigate
    }: {
        manuscripts: Manuscript[];
        onNavigate: (view: "dashboard" | "grants" | "manuscripts" | "add-grant" | "add-manuscript") => void;
    } = $props();
</script>

<section class="view">
    <header class="page-header">
        <div>
            <h1>Grants</h1>
            <p>Grant applications and funding decisions</p>
        </div>

        <button
            class="primary"
            onclick={() => onNavigate("add-grant")}
        >
            Add grant
        </button>
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
                        <th>Amount</th>
                        <th>Status</th>
                    </tr>
                </thead>

                <tbody>
                    {#each grants as grant}
                        <tr>
                            <td class="main-cell">{grant.name}</td>
                            <td>{grant.funder}</td>
                            <td>{grant.deadline}</td>
                            <td>{grant.amount}</td>
                            <td>
                                <span class="status">
                                    {grant.status}
                                </span>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    {/if}
</section>

<style>
    .view {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .page-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 1rem;
    }

    .page-header h1 {
        margin: 0;
    }

    .page-header p {
        margin: 0.25rem 0 0;
        opacity: 0.65;
    }

    button {
        padding: 0.65rem 1rem;
        border: 1px solid #444;
        border-radius: 0.45rem;
        background: #222;
        color: inherit;
        font: inherit;
        cursor: pointer;
    }

    button.primary {
        font-weight: 600;
    }

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

    th {
        font-size: 0.85rem;
        opacity: 0.7;
    }

    tr:last-child td {
        border-bottom: none;
    }

    .main-cell {
        font-weight: 600;
    }

    .status {
        font-size: 0.9rem;
        font-weight: 600;
    }

    .empty-state {
        padding: 2rem;
        border: 1px dashed #444;
        border-radius: 0.6rem;
        text-align: center;
    }

    .empty-state h2 {
        margin-top: 0;
    }

    .empty-state p {
        margin-bottom: 0;
        opacity: 0.7;
    }
</style>