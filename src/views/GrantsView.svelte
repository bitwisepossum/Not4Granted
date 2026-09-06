<script lang="ts">
    import "../app.css";
    import type { Grant, GrantStatus } from "../types";

    let {
        grants,
        onNavigate
    }: {
        grants: Grant[];
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