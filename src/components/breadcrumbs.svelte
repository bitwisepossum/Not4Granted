<script lang="ts">
    import { page } from "$app/state";
    import "../styles/app.css";

    const labels: Record<string, string> = {
        grants: "Grants",
        manuscripts: "Manuscripts",
        new: "New"
    };

    let parts = $derived(
        page.url.pathname
            .split("/")
            .filter(Boolean)
    );
</script>

<nav class="breadcrumbs" aria-label="Breadcrumb">
    <a href="/">Dashboard</a>

    {#each parts as part, i}
        <span>/</span>

        {#if i === parts.length - 1}
            <span class="current">
                {labels[part] ?? part}
            </span>
        {:else}
            <a href={"/" + parts.slice(0, i + 1).join("/")}>
                {labels[part] ?? part}
            </a>
        {/if}
    {/each}
</nav>