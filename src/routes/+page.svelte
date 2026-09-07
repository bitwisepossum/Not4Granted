<script lang="ts">
    import { onMount } from "svelte";

    import DashboardView from "../views/DashboardView.svelte";
    import { getGrants, getManuscripts } from "../components/api";

    import type { Grant, Manuscript } from "../types";

    let grants = $state<Grant[]>([]);
    let manuscripts = $state<Manuscript[]>([]);

    let isLoading = $state(true);
    let error = $state<string | undefined>(undefined);

    onMount(async () => {
        try {
            grants = await getGrants();
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
    {manuscripts}
    {isLoading}
    {error}
/>