<script lang="ts">
    import { page } from "$app/state";
    import "../styles/app.css";


</script>

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
            <a
                class:active={page.url.pathname.startsWith("/about")}
                class="nav-link"
                href="/about"
            >
                About
            </a>
            local database target
        </div>