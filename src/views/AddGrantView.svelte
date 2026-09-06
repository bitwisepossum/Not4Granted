<script lang="ts">
    import type { Grant, GrantStatus } from "../types";

    let name: string = $state<string>("");
    let funder: string = $state<string>("");
    let deadline = $state<string>("");
    let amount: string = $state<string>("");
    let status = $state<GrantStatus>("Planning");

    let { 
        onCancel,
        onSubmit
    }: {
        onCancel: () => void;
        onSubmit: (grant: Grant) => void;
    } = $props();

    function submit() {
        const newGrant: Grant = {
            id: Date.now(),
            name,
            funder,
            deadline,
            amount,
            status
        };

        onSubmit(newGrant);
    }


</script>

<section class="view">
    <header>
        <h1>Add grant</h1>
        <p>Create a new grant application entry.</p>
    </header>

    <form
        onsubmit={(event) => {
            event.preventDefault();
            submit();
        }}
    >
        <div class="field">
            <label for="grant-name">Name</label>
            <input
                id="grant-name"
                type="text"
                bind:value={name}
                required
            />
        </div>

        <div class="field">
            <label for="grant-funder">Funder</label>
            <input
                id="grant-funder"
                type="text"
                bind:value={funder}
                required
            />
        </div>

        <div class="form-row">
            <div class="field">
                <label for="grant-deadline">Deadline</label>
                <input
                    id="grant-deadline"
                    type="date"
                    bind:value={deadline}
                />
            </div>

            <div class="field">
                <label for="grant-amount">Amount</label>
                <input
                    id="grant-amount"
                    type="text"
                    bind:value={amount}
                    placeholder="15000"
                />
            </div>
        </div>

        <div class="field">
            <label for="grant-status">Status</label>

            <select
                id="grant-status"
                bind:value={status}
            >
                <option value="Planning">Planning</option>
                <option value="Submitted">Submitted</option>
                <option value="Accepted">Accepted</option>
                <option value="Rejected">Rejected</option>
            </select>
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
                Save grant
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

    .form-row {
        display: grid;
        grid-template-columns: 1fr 1fr;
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