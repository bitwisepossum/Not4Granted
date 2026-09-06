<script lang="ts">
    import type { Manuscript, ManuscriptStatus } from "../types";

    let title: string = $state<string>("");
    let journal: string = $state<string>("");
    let status = $state<ManuscriptStatus>("Idea");
    let nextAction: string = $state<string>("");

    let { 
        onCancel,
        onSubmit
    }: {
        onCancel: () => void;
        onSubmit: (manuscript: Manuscript) => void;
    } = $props();

    function submit() {
        const newManuscript: Manuscript = {
            id: Date.now(),
            title,
            journal,
            status,
            nextAction
        };

        onSubmit(newManuscript);
    }
</script>

<section class="view">
    <header>
        <h1>Add manuscript</h1>
        <p>Create a new manuscript entry.</p>
    </header>

    <form
        onsubmit={(event) => {
            event.preventDefault();
            submit();
        }}
    >
        <div class="field">
            <label for="manuscript-title">Title</label>
            <input
                id="manuscript-title"
                type="text"
                bind:value={title}
                required
            />
        </div>

        <div class="field">
            <label for="manuscript-journal">Journal</label>
            <input
                id="manuscript-journal"
                type="text"
                bind:value={journal}
            />
        </div>

        <div class="field">
            <label for="manuscript-status">Status</label>

            <select
                id="manuscript-status"
                bind:value={status}
            >
                <option value="Idea">Idea</option>
                <option value="Drafting">Drafting</option>
                <option value="Submitted">Submitted</option>
                <option value="Revision">Revision</option>
                <option value="Accepted">Accepted</option>
            </select>
        </div>

        <div class="field">
            <label for="manuscript-next-action">Next action</label>

            <input
                id="manuscript-next-action"
                type="text"
                bind:value={nextAction}
                placeholder="Revise methods section"
            />
        </div>

        <div class="actions">
            <button
                type="button"
                onclick={onCancel}
            >
                Cancel
            </button>

            <button
                class="primary"
                type="submit"
            >
                Save manuscript
            </button>
        </div>
    </form>
</section>

<style>
    .view {
        max-width: 720px;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    header h1 {
        margin: 0;
    }

    header p {
        margin: 0.25rem 0 0;
        opacity: 0.65;
    }

    form {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .field {
        display: flex;
        flex-direction: column;
        gap: 0.4rem;
    }

    label {
        font-size: 0.9rem;
        font-weight: 600;
    }

    input,
    select {
        padding: 0.7rem;
        border: 1px solid #444;
        border-radius: 0.4rem;
        background: #181818;
        color: inherit;
        font: inherit;
    }

    .actions {
        display: flex;
        justify-content: flex-end;
        gap: 0.75rem;
        margin-top: 1rem;
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

    button.primary {
        font-weight: 600;
    }
</style>