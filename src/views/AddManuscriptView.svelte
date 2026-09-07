<script lang="ts">
    import "../styles/app.css";
    import type { ManuscriptStatus, NewManuscript } from "../types";

    let title = $state("");
    let shortName = $state("");
    let journal = $state("");
    let status = $state<ManuscriptStatus>("Idea");
    let nextAction = $state("");

    let submittedAt = $state("");
    let decisionAt = $state("");
    let publishedAt = $state("");

    let doi = $state("");
    let notes = $state("");

    let {
        onCancel,
        onSubmit
    }: {
        onCancel: () => void;
        onSubmit: (manuscript: NewManuscript) => void;
    } = $props();

    function closeDatePicker(event: Event) {
        const input = event.currentTarget as HTMLInputElement;
        setTimeout(() => input.blur(), 0);
    }

    function submit() {
        const manuscript: NewManuscript = {
            title: title.trim(),
            shortName: shortName.trim() || undefined,
            journal: journal.trim() || undefined,
            status,
            nextAction: nextAction.trim() || undefined,
            submittedAt: submittedAt || undefined,
            decisionAt: decisionAt || undefined,
            publishedAt: publishedAt || undefined,
            doi: doi.trim() || undefined,
            notes: notes.trim() || undefined
        };

        onSubmit(manuscript);
    }
</script>

<section class="view">
    <header>
        <div>
            <h1>Add manuscript</h1>
            <p>Create a new manuscript entry.</p>
        </div>
    </header>

    <form
        class="form"
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

        <div class="form-row">
            <div class="field">
                <label for="manuscript-short-name">Short name</label>
                <input
                    id="manuscript-short-name"
                    type="text"
                    bind:value={shortName}
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
        </div>

        <div class="form-row">
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
                    <option value="Published">Published</option>
                    <option value="Rejected">Rejected</option>
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
        </div>

        <div class="form-row">
            <div class="field">
                <label for="manuscript-submitted-at">Submitted</label>
                <input
                    id="manuscript-submitted-at"
                    type="date"
                    bind:value={submittedAt}
                    onchange={closeDatePicker}
                />
            </div>

            <div class="field">
                <label for="manuscript-decision-at">Decision date</label>
                <input
                    id="manuscript-decision-at"
                    type="date"
                    bind:value={decisionAt}
                    onchange={closeDatePicker}
                />
            </div>
        </div>

        <div class="field">
            <label for="manuscript-published-at">Published</label>
            <input
                id="manuscript-published-at"
                type="date"
                bind:value={publishedAt}
                onchange={closeDatePicker}
            />
        </div>

        <div class="field">
            <label for="manuscript-doi">DOI</label>
            <input
                id="manuscript-doi"
                type="text"
                bind:value={doi}
                placeholder="10.xxxx/..."
            />
        </div>

        <div class="field">
            <label for="manuscript-notes">Notes</label>
            <textarea
                id="manuscript-notes"
                rows="5"
                bind:value={notes}
            ></textarea>
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