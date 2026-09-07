<script lang="ts">
    import { onMount } from "svelte";
    import GrantsView from "../../views/GrantsView.svelte";
    import { getGrants } from "../../components/api";
    import type { Grant, GrantStatus, View } from "../../types";

    let grants = $state<Grant[]>([]);

    onMount(async () => {
        grants = await getGrants();
    });
</script>

<svelte:head>
    <title>Grants - Not4Granted</title>
</svelte:head>

<GrantsView {grants} />