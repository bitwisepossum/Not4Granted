<script lang="ts">
    import "../../styles/app.css";

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
</script>

<section class="about">
    <header class="about-header">
        <h1>About Not4Granted</h1>
    </header>

    <div class="about-card">
        <section class="intro">
            <h2>Not4Granted</h2>

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

        <footer class="license">
            Licensed under the
            <strong>European Union Public Licence 1.2</strong>.
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

    .intro h2 {
        margin: 0 0 0.5rem;
    }

    .intro p {
        margin: 0;
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

    .license {
        margin-top: 2rem;
        padding-top: 1rem;
        border-top: 1px solid #ddd;
        font-size: 0.9rem;
    }
</style>