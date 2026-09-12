<script lang="ts">
    import { onMount } from "svelte";

    import DashboardView from "../views/DashboardView.svelte";
    import { getGrants, getManuscripts, getFilteredGrants } from "../components/api";

    import type { Grant, Manuscript, GrantQuery } from "../types";

    let grants = $state<Grant[]>([]);
    let active_grants = $state<Grant[]>([]);
    let manuscripts = $state<Manuscript[]>([]);
    let query = $state<GrantQuery>({
        statuses: ["Planning", "Submitted"],
        funders: [],
        search: undefined,
        sortBy: undefined,
        sortDirection: undefined
    });

    let isLoading = $state(true);
    let error = $state<string | undefined>(undefined);

    onMount(async () => {
        try {
            grants = await getGrants();
            active_grants = await getFilteredGrants(query);
            manuscripts = await getManuscripts();
            console.log("Grants:", grants); //debug
        } catch (err) {
            error = `Failed to load dashboard: ${err}`;
        } finally {
            isLoading = false;
            console.log("Manuscripts:", manuscripts); //debug
        }
    });
</script>

<svelte:head>
    <title>Dashboard | Not4Granted</title>
</svelte:head>

<DashboardView
    {grants}
    {manuscripts}
    {isLoading}
    {error}
/>