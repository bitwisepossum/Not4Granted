<script lang="ts">
    import "../styles/app.css";
    import SideBar from "../components/sidebar.svelte";
    import Breadcrumbs from "../components/breadcrumbs.svelte";

    let { children } = $props();
</script>

<div class="app-shell">
    <aside class="sidebar">
        <SideBar />
    </aside>

    <main>
        <Breadcrumbs />

        {@render children()}
    </main>
</div>