<script lang="ts">
    import { onMount } from "svelte";
    import { settingsState } from "../state/settings.svelte";
    import "../styles/app.css";
    import SideBar from "../components/sidebar.svelte";
    import Breadcrumbs from "../components/breadcrumbs.svelte";

    let { children } = $props();

    onMount(() => {
        void settingsState.load();
    })
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