<script lang="ts">
    import { goto } from "$app/navigation";
    import AddManuscriptView from "../../../views/AddManuscriptView.svelte";
    import { addManuscript } from "../../../components/api";
    import type { NewManuscript, ManuscriptStatus } from "../../../types";

    async function handleSubmit(manuscript: NewManuscript) {
        await addManuscript(manuscript);
        await goto("/manuscripts"); 
    }

    function handleCancel() {
        goto("/manuscripts");
    }
</script>

<svelte:head>
    <title>Add Manuscript - Not4Granted</title>
</svelte:head>

<AddManuscriptView onSubmit={handleSubmit} onCancel={handleCancel} />