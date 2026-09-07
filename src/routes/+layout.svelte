<script lang="ts">
    import "../styles/app.css";
    import { page } from "$app/state"

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
            <a
                class:active={page.url.pathname === "/"}
                class="nav-link"
                href="/"
            >
                Dashboard
            </a>

            <a
                class:active={page.url.pathname.startsWith("/grants")}
                class="nav-link"
                href="/grants"
            >
                Grants
            </a>

            <a
                class:active={page.url.pathname.startsWith("/manuscripts")}
                class="nav-link"
                href="/manuscripts"
            >
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