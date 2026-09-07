<script lang="ts">
    import { onMount } from "svelte";
    import ManuscriptsView from "../../views/ManuscriptsView.svelte";
    import { getManuscripts } from "../../components/api";
    import type { Manuscript, ManuscriptStatus } from "../../types";

    let manuscripts = $state<Manuscript[]>([]);

    onMount(async () => {
        manuscripts = await getManuscripts();
    });
</script>

<svelte:head>
    <title>Manuscripts - Not4Granted</title>
</svelte:head>

<ManuscriptsView {manuscripts} />