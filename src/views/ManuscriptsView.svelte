<script lang="ts">
    import { goto } from "$app/navigation";
    import "../styles/app.css";
    import "../styles/data-view.css";
    import { manuscriptStatuses, type Manuscript, type ManuscriptStatus} from "../types";
    import { formatDate } from "../components/format";
    import { settingsState } from "../state/settings.svelte";
    import { getFilteredManuscripts, getManuscripts } from "../components/api";

    let {
        manuscripts
    }: {
        manuscripts: Manuscript[];
    } = $props();

    let expandedManuscriptId = $state<number | undefined>(undefined);

    let filterQuery = $state({
        search: "",
        statuses: [] as ManuscriptStatus[],
        journal: "",
        sortBy: "title" as const,
        sortDirection: "asc" as const
    });

    function toggleManuscript(
        id: number,
        status: ManuscriptStatus
    ) {
        if (expandedManuscriptId === id) {
            expandedManuscriptId = undefined;
        } else {
            expandedManuscriptId = id;
        }
    }

    function statusClass(status: string): string {
        return status.toLowerCase();
    }
    
    function gotoManuscript(id: number) {
        goto(`/manuscripts/${id}`);
    }

    async function handleFilterSubmit(event: SubmitEvent) {      
        event.preventDefault();
    
        const query = {
            statuses: filterQuery.statuses,
            journal: filterQuery.journal,
            search: filterQuery.search,
            sortBy: filterQuery.sortBy,
            sortDirection: filterQuery.sortDirection
        };

        manuscripts = await getFilteredManuscripts(query);

        console.log("Filter applied:", query);
    }

    async function handleFilterReset() {
        filterQuery = {
            search: "",
            statuses: [] as ManuscriptStatus[],
            journal: "",
            sortBy: "title" as const,
            sortDirection: "asc" as const
        };
        manuscripts = await getManuscripts();
    }
</script>

<section class="view">
    <header class="page-header">
        <div>
            <h1>Manuscripts</h1>
            <p>Publication progress and next actions</p>
        </div>

        <a class="button-link" href="/manuscripts/new">Add manuscript</a>
    </header>

    <form class="filter-bar" onsubmit={handleFilterSubmit} onreset={handleFilterReset}>
        <label class="filter-search">
            <span class="visually-hidden">Search manuscripts</span>
            <input
                type="search"
                bind:value={filterQuery.search}
                placeholder="Search manuscripts"
            />
        </label>

        <details class="filter-menu">
            <summary>Filters</summary>

            <div class="filter-menu-content">
                <fieldset>
                    <legend>Status</legend>

                    <div class="filter-options">
                        {#each manuscriptStatuses as status}
                            <label>
                                <input
                                    type="checkbox"
                                    bind:group={filterQuery.statuses}
                                    value={status}
                                />
                                {status}
                            </label>
                        {/each}
                    </div>
                </fieldset>

                <label class="filter-field">
                    <span>Journal</span>
                    <input
                        type="text"
                        bind:value={filterQuery.journal}
                        placeholder="Any journal"
                    />
                </label>
            </div>
        </details>

        <label class="filter-sort">
            <span class="visually-hidden">Sort manuscripts by</span>
            <select bind:value={filterQuery.sortBy} aria-label="Sort manuscripts by">
                <option value="title">Title</option>
                <option value="status">Status</option>
                <option value="journal">Journal</option>
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

    {#if manuscripts.length === 0}
        <div class="empty-state">
            <h2>No manuscripts</h2>
            <p>Add the first manuscript to start tracking it.</p>
        </div>
    {:else}
        <div class="data-table table-wrapper">
            <table>
                <thead>
                    <tr>
                        <th class="disclosure-column"></th>
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
                            class="data-interactive"
                            class:expanded={
                                expandedManuscriptId === manuscript.id
                            }
                            onclick={() => gotoManuscript(manuscript.id)}
                        >
                            <td class="disclosure-cell">
                                <span class="disclosure">
                                    {expandedManuscriptId === manuscript.id
                                        ? "⌄"
                                        : "›"}
                                </span>
                            </td>

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
                    {/each}
                </tbody>
            </table>
        </div>
    {/if}
</section>

<style>
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