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
            <a class="nav-link" href="/">
                Dashboard
            </a>

            <a class="nav-link" href="/grants">
                Grants
            </a>

            <a class="nav-link" href="/manuscripts">
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