<script lang="ts">
    import type { GrantStatus, NewGrant } from "../types";
    import { convertCurrency } from "../components/format";

    let name = $state("");
    let funder = $state("");
    let callName = $state("");
    let status = $state<GrantStatus>("Planning");

    // Form values are entered as currency units, then converted to cents on submit.
    let amountRequested = $state<number | undefined>(undefined);
    let amountReceived = $state<number | undefined>(undefined);
    let currency = $state("EUR");

    let deadline = $state("");
    let submittedAt = $state("");
    let decisionAt = $state("");

    let notes = $state("");

    type GrantDateFields = "deadline" | "submittedAt" | "decisionAt";

    let {
        onCancel,
        onSubmit
    }: {
        onCancel: () => void;
        onSubmit: (grant: NewGrant) => void;
    } = $props();

    function closeDatePicker(event: Event) {
        const input = event.currentTarget as HTMLInputElement;
        setTimeout(() => input.blur(), 0);
    }

    function submit() {
        const requested =
            amountRequested === undefined
                ? undefined
                : convertCurrency(amountRequested, true);
        const received =
            amountReceived === undefined
                ? undefined
                : convertCurrency(amountReceived, true);

        if (amountRequested !== undefined && requested === undefined) {
            alert("Please enter a valid requested amount.");
            return;
        }

        if (amountReceived !== undefined && received === undefined) {
            alert("Please enter a valid received amount.");
            return;
        }

        const grant: NewGrant = {
            name: name.trim(),
            funder: funder.trim(),
            callName: callName.trim() || undefined,
            status,
            amountRequested: requested,
            amountReceived: received,
            currency: currency.trim().toUpperCase() || "EUR",
            deadline: deadline || undefined,
            submittedAt: submittedAt || undefined,
            decisionAt: decisionAt || undefined,
            notes: notes.trim() || undefined
        };

        onSubmit(grant);
    }

    function clearDate(field: GrantDateFields) {
        switch (field) {
            case "deadline":
                deadline = "";
                break;
            case "submittedAt":
                submittedAt = "";
                break;
            case "decisionAt":
                decisionAt = "";
                break;
        }
    }
</script>

<section class="view">
    <header>
        <div>
            <h1>Add grant</h1>
            <p>Create a new grant application entry.</p>
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
            <label for="grant-name">Name</label>
            <input
                id="grant-name"
                type="text"
                bind:value={name}
                required
            />
        </div>

        <div class="form-row">
            <div class="field">
                <label for="grant-funder">Funder</label>
                <input
                    id="grant-funder"
                    type="text"
                    bind:value={funder}
                    required
                />
            </div>

            <div class="field">
                <label for="grant-call-name">Call name</label>
                <input
                    id="grant-call-name"
                    type="text"
                    bind:value={callName}
                />
            </div>
        </div>

        <div class="form-row">
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

            <div class="field">
                <label for="grant-currency">Currency</label>
                <input
                    id="grant-currency"
                    type="text"
                    maxlength="3"
                    bind:value={currency}
                />
            </div>
        </div>

        <div class="form-row">
            <div class="field">
                <label for="grant-amount-requested">Amount requested</label>
                <input
                    id="grant-amount-requested"
                    type="number"
                    min="0"
                    step="0.01"
                    bind:value={amountRequested}
                    placeholder="15000"
                />
            </div>

            <div class="field">
                <label for="grant-amount-received">Amount received</label>
                <input
                    id="grant-amount-received"
                    type="number"
                    min="0"
                    step="0.01"
                    bind:value={amountReceived}
                />
            </div>
        </div>

        <div class="form-row">
            <div class="field">
                <label for="grant-deadline">Deadline</label>
                <input
                    id="grant-deadline"
                    type="date"
                    bind:value={deadline}
                    onchange={closeDatePicker}
                />
                <button class="item-action" type="button" onclick={() => clearDate("deadline")}>
                    Clear
                </button>
            </div>

            <div class="field">
                <label for="grant-submitted-at">Submitted</label>
                <input
                    id="grant-submitted-at"
                    type="date"
                    bind:value={submittedAt}
                    onchange={closeDatePicker}
                />
                <button class="item-action" type="button" onclick={() => clearDate("submittedAt")}>
                    Clear
                </button>
            </div>
        </div>

        <div class="field">
            <label for="grant-decision-at">Decision date</label>
            <input
                id="grant-decision-at"
                type="date"
                bind:value={decisionAt}
                onchange={closeDatePicker}
            />
            <button class="item-action" type="button" onclick={() => clearDate("decisionAt")}>
                Clear
            </button>
        </div>

        <div class="field">
            <label for="grant-notes">Notes</label>
            <textarea
                id="grant-notes"
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
                Save grant
            </button>
        </div>
    </form>
</section>