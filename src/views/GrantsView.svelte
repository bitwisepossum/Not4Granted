<script lang="ts">
    import "../styles/app.css";
    import "../styles/data-view.css";
    import { convertCurrency, formatDate } from "../components/format";
    import { grantStatuses, type Grant, type GrantStatus, type GrantQuery } from "../types";
    import { settingsState } from "../state/settings.svelte";
    import { getFilteredGrants, getGrants } from "../components/api";

    let {
        grants
    }: {
        grants: Grant[];
    } = $props();

    let expandedGrantId = $state<number | undefined>(undefined);
    let selectedStatus = $state<GrantStatus>("Planning");

    let filterQuery = $state<GrantQuery>({
        statuses: [],
        funder: "",
        search: "",
        sortBy: "deadline",
        sortDirection: "asc"
    });

    function toggleGrant(id: number, status: GrantStatus) {
        if (expandedGrantId === id) {
            expandedGrantId = undefined;
        } else {
            expandedGrantId = id;
            selectedStatus = status;
        }
    }

    function statusClass(status: string): string {
        return status.toLowerCase();
    }

    async function handleFilterSubmit(event: SubmitEvent) {
        event.preventDefault();

        const query: GrantQuery = {
            statuses: filterQuery.statuses,
            funder: filterQuery.funder,
            search: filterQuery.search,
            sortBy: filterQuery.sortBy,
            sortDirection: filterQuery.sortDirection
        };

        grants = await getFilteredGrants(query);
    }

    async function handleFilterReset() {
        filterQuery = {
            search: "",
            statuses: [],
            funder: "",
            sortBy: "deadline",
            sortDirection: "asc"
        };
        grants = await getGrants();
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

    <form class="filter-bar" onsubmit={handleFilterSubmit} onreset={handleFilterReset}>
        <label class="filter-search">
            <span class="visually-hidden">Search grants</span>
            <input
                type="search"
                bind:value={filterQuery.search}
                placeholder="Search grants"
            />
        </label>

        <details class="filter-menu">
            <summary>Filters</summary>

            <div class="filter-menu-content">
                <fieldset>
                    <legend>Status</legend>

                    <div class="filter-options">
                        {#each grantStatuses as status}
                            <label>
                                <input
                                    type="checkbox"
                                    value={status}
                                    bind:group={filterQuery.statuses}
                                />
                                {status}
                            </label>
                        {/each}
                    </div>
                </fieldset>

                <label class="filter-field">
                    <span>Funder</span>
                    <input
                        type="text"
                        bind:value={filterQuery.funder}
                        placeholder="Any funder"
                    />
                </label>
            </div>
        </details>

        <label class="filter-sort">
            <span class="visually-hidden">Sort grants by</span>
            <select bind:value={filterQuery.sortBy} aria-label="Sort grants by">
                <option value="deadline">Deadline</option>
                <option value="name">Name</option>
                <option value="status">Status</option>
                <option value="funder">Funder</option>
            </select>
        </label>

        <label class="filter-direction">
            <span class="visually-hidden">Sort direction</span>
            <select bind:value={filterQuery.sortDirection} aria-label="Sort direction">
                <option value="asc">Ascending</option>
                <option value="desc">Descending</option>
            </select>
        </label>

        <button class="secondary" type="submit">Apply</button>
        <button class="link-button" type="reset">Reset</button>
    </form>

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
                            <td>{grant.funder}</td>
                            <td>{formatDate(grant.deadline, settingsState.current.locale) ?? "—"}</td>

                            <td>
                                {grant.amountRequested !== undefined
                                    ? `${convertCurrency(grant.amountRequested)} ${grant.currency}`
                                    : "—"}
                            </td>

                            <td>
                                {grant.amountReceived !== undefined
                                    ? `${convertCurrency(grant.amountReceived)} ${grant.currency}`
                                    : "—"}
                            </td>

                            <td class="data-main">
                                <span class={`status ${statusClass(grant.status)}`}>{grant.status}</span>
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
