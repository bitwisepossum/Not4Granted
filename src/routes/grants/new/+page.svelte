<script lang="ts">
    import { goto } from "$app/navigation";
    import { addGrant } from "../../../components/api";
    import AddGrantView from "../../../views/AddGrantView.svelte";
    import type { NewGrant } from "../../../types";

    async function handleSubmit(grant: NewGrant) {
        try {
            await addGrant(grant);
            goto("/grants");
        } catch (err) {
            console.error("Failed to add grant:", err);
        }
    }

    function handleCancel() {
        goto("/grants");
    }
</script>

<svelte:head>
    <title>Add Grant - Not4Granted</title>
</svelte:head>

<AddGrantView onSubmit={handleSubmit} onCancel={handleCancel} />