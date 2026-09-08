<script lang="ts">
    import "../../styles/app.css";
    import { getVersion } from "@tauri-apps/api/app";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { onMount } from "svelte";

    let version = $state("");

    const technologies = [
        {
            name: "Tauri",
            logo: "/tauri.svg"
        },
        {
            name: "Svelte",
            logo: "/svelte.svg"
        },
        {
            name: "Vite",
            logo: "/vite.svg"
        }
    ];

    onMount(async () => {
        version = await getVersion();
    });

    async function openGitHub() {
        await openUrl("https://github.com/bitwisepossum/not4granted");
    }
</script>

<section class="about">
    <header class="about-header">
        <h1>About Not4Granted</h1>
    </header>

    <div class="about-card">
        <section class="intro">
            <div class="title-row">
                <h2>Not4Granted</h2>

                {#if version}
                    <span class="version">
                        v{version}
                    </span>
                {/if}
            </div>

            <p>
                Not4Granted tracks grant applications, funding decisions,
                and manuscript progress, mostly so rejection can be
                quantified properly.
            </p>
        </section>

        <section class="about-section">
            <h3>Technology</h3>

            <div class="tech-stack">
                {#each technologies as technology}
                    <div class="tech">
                        <img
                            src={technology.logo}
                            alt={technology.name}
                        />

                        <span>{technology.name}</span>
                    </div>
                {/each}
            </div>
        </section>

        <section class="about-section">
            <h3>Source code</h3>

            <button
                class="about-link"
                type="button"
                onclick={openGitHub}
            >
                GitHub repository
            </button>
        </section>

        <footer class="license">
            <p>Copyright © 2026 <button class="about-link" type="button" onclick={() => openUrl("https://github.com/bitwisepossum")}>Bitwisepossum</button></p>
            <br />
            <p>Licensed under the
            <button
                class="about-link"
                type="button"
                onclick={() =>
                    openUrl(
                        "https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12"
                    )}
            >
                European Union Public Licence 1.2 or later (EUPL-1.2-or-later)
            </button></p>
        </footer>
    </div>
</section>

<style>
    .about {
        max-width: 760px;
    }

    .about-header {
        margin-bottom: 1.5rem;
    }

    .about-header h1 {
        margin: 0;
    }

    .about-card {
        padding: 2rem;
        border: 1px solid #333;
        border-radius: 0.75rem;
        background: white;
    }

    .title-row {
        display: flex;
        align-items: baseline;
        gap: 0.75rem;
    }

    .intro h2 {
        margin: 0;
    }

    .version {
        font-size: 0.85rem;
        color: #777;
    }

    .intro p {
        margin: 0.5rem 0 0;
        max-width: 620px;
        line-height: 1.5;
    }

    .about-section {
        margin-top: 2rem;
    }

    .about-section h3 {
        margin: 0 0 1rem;
        font-size: 1rem;
    }

    .tech-stack {
        display: flex;
        gap: 2rem;
        flex-wrap: wrap;
    }

    .tech {
        display: flex;
        align-items: center;
        gap: 0.65rem;
    }

    .tech img {
        width: 40px;
        height: 40px;
        object-fit: contain;
    }

    .tech span {
        font-weight: 600;
    }

    .about-link {
        padding: 0;
        border: 0;
        background: none;
        color: inherit;
        font: inherit;
        text-decoration: underline;
        cursor: pointer;
    }

    .license {
        margin-top: 2rem;
        padding-top: 1rem;
        border-top: 1px solid #ddd;
        font-size: 0.9rem;
    }
</style>