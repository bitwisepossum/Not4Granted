<script lang="ts">
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

<style>
    .view {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .page-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 1rem;
    }

    .page-header h1 {
        margin: 0;
    }

    .page-header p {
        margin: 0.25rem 0 0;
        opacity: 0.65;
    }

    button {
        padding: 0.65rem 1rem;
        border: 1px solid #444;
        border-radius: 0.45rem;
        background: #222;
        color: inherit;
        font: inherit;
        cursor: pointer;
    }

    .cards {
        display: grid;
        gap: 1rem;
    }

    .card {
        padding: 1rem;
        border: 1px solid #333;
        border-radius: 0.6rem;
    }

    .card-main {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        gap: 1rem;
    }

    .card h2 {
        margin: 0;
        font-size: 1rem;
    }

    .card p {
        margin: 0.3rem 0 0;
        opacity: 0.65;
    }

    .status {
        white-space: nowrap;
        font-weight: 600;
    }

    .next-action {
        display: flex;
        flex-direction: column;
        gap: 0.2rem;
        margin-top: 1rem;
        padding-top: 1rem;
        border-top: 1px solid #333;
    }

    .next-action span {
        font-size: 0.8rem;
        opacity: 0.6;
    }

    .empty-state {
        padding: 2rem;
        border: 1px dashed #444;
        border-radius: 0.6rem;
        text-align: center;
    }

    .empty-state h2 {
        margin-top: 0;
    }

    .empty-state p {
        margin-bottom: 0;
        opacity: 0.7;
    }
</style>