<script lang="ts">
    import { onMount } from "svelte";

    import DashboardView from "../views/DashboardView.svelte";
    import { getGrants, getManuscripts, getFilteredGrants, getFilteredManuscripts } from "../components/api";

    import type { Grant, Manuscript, GrantQuery, ManuscriptQuery } from "../types";

    let grants = $state<Grant[]>([]);
    let active_grants = $state<Grant[]>([]);
    let manuscripts = $state<Manuscript[]>([]);
    let active_manuscripts = $state<Manuscript[]>([]);

    let active_grants_query = $state<GrantQuery>({
        statuses: ["Planning", "Submitted"],
        funders: [],
        search: undefined,
        sortBy: undefined,
        sortDirection: undefined
    });

    let active_manuscripts_query = $state<ManuscriptQuery>({
        statuses: ["Idea", "Drafting", "Submitted", "Revision"],
        search: undefined,
        sortBy: undefined,
        sortDirection: undefined
    });

    let isLoading = $state(true);
    let error = $state<string | undefined>(undefined);

    onMount(async () => {
        try {
            grants = await getGrants();
            active_grants = await getFilteredGrants(active_grants_query);
            active_manuscripts = await getFilteredManuscripts(active_manuscripts_query);
            manuscripts = await getManuscripts();
        } catch (err) {
            error = `Failed to load dashboard: ${err}`;
        } finally {
            isLoading = false;
        }
    });
</script>

<svelte:head>
    <title>Dashboard | Not4Granted</title>
</svelte:head>

<DashboardView
    {grants}
    {active_grants}
    {manuscripts}
    {active_manuscripts}
    {isLoading}
    {error}
/>