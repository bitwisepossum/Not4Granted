<script lang="ts">
    import "../app.css";
    import type { Manuscript } from "../types";

    let {
        manuscripts,
        onNavigate
    }: {
        manuscripts: Manuscript[];
        onNavigate: (view: "dashboard" | "grants" | "manuscripts" | "add-grant" | "add-manuscript") => void;
    } = $props();
</script>

<section class="view">
    <header class="page-header">
        <div>
            <h1>Manuscripts</h1>
            <p>Publication progress and next actions</p>
        </div>

        <button
            class="primary"
            onclick={() => onNavigate("add-manuscript")}
        >
            Add manuscript
        </button>
    </header>

    {#if manuscripts.length === 0}
        <div class="empty-state">
            <h2>No manuscripts</h2>
            <p>Add the first manuscript to start tracking it.</p>
        </div>
    {:else}
        <div class="cards">
            {#each manuscripts as manuscript}
                <article class="card">
                    <div class="card-main">
                        <div>
                            <h2>{manuscript.title}</h2>
                            <p>{manuscript.journal}</p>
                        </div>

                        <span class="status">
                            {manuscript.status}
                        </span>
                    </div>

                    <div class="next-action">
                        <span>Next action</span>
                        <strong>{manuscript.nextAction}</strong>
                    </div>
                </article>
            {/each}
        </div>
    {/if}
</section>