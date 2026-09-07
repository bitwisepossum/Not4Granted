<script lang="ts">
    import "../app.css";

    let { children } = $props();
</script>

<div class="app-shell">
    <aside class="sidebar">
        <div class="brand">
            <div class="brand-mark">N4G</div>

            <div>
                <strong>Not4Granted</strong>
                <span>Research tracker</span>
            </div>
        </div>

        <nav>
            <a href="/">
                Dashboard
            </a>

            <a href="/grants">
                Grants
            </a>

            <a href="/manuscripts">
                Manuscripts
            </a>
        </nav>

        <div class="sidebar-footer">
            local database target
        </div>
    </aside>

    <main>
        {@render children()}
    </main>
</div>