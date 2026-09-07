<script lang="ts">
    import { goto } from "$app/navigation";
    import { addGrant } from "../../../components/api";
    import AddGrantView from "../../../views/AddGrantView.svelte";
    import type { NewGrant, GrantStatus } from "../../../types";

    async function handleSubmit(grant: NewGrant) {
        await addGrant(grant);
        await goto("/grants");
    }

    function handleCancel() {
        goto("/grants");
    }
</script>

<svelte:head>
    <title>Add Grant - Not4Granted</title>
</svelte:head>

<AddGrantView onSubmit={handleSubmit} onCancel={handleCancel} />