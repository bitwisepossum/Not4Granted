<script lang="ts">
    import "../app.css";
    import type { Grant, Manuscript } from "../types";

    let  {
        grants,
        manuscripts
    }: {
        grants: Grant[];
        manuscripts: Manuscript[];
    } = $props();

    let appliedGrants = $derived(
        grants.filter(
            (grant) =>
                grant.status === "Submitted" ||
                grant.status === "Accepted" ||
                grant.status === "Rejected"
        )
    );

    let acceptedGrants = $derived(
        grants.filter(
            (grant) => grant.status === "Accepted"
        )
    );

    let rejectedGrants = $derived(
        grants.filter(
            (grant) => grant.status === "Rejected"
        )
    );

    let decidedGrants = $derived(
        grants.filter(
            (grant) =>
                grant.status === "Accepted" ||
                grant.status === "Rejected"
        )
    );

    let inProgressGrants = $derived(
        grants.filter(
            (grant) =>
                grant.status === "Planning" ||
                grant.status === "Submitted"
        )
    );

    let successRate = $derived(
        decidedGrants.length === 0
            ? 0
            : Math.round(
                (acceptedGrants.length / decidedGrants.length) * 100
            )
    );
</script>

<section>
    <h1>Dashboard</h1>

    <div>
        <strong>Applied:</strong>
        {appliedGrants.length}
    </div>

    <div>
        <strong>Accepted:</strong>
        {acceptedGrants.length}
    </div>

    <div>
        <strong>Rejected:</strong>
        {rejectedGrants.length}
    </div>

    <div>
        <strong>In progress:</strong>
        {inProgressGrants.length}
    </div>

    <div>
        <strong>Success rate:</strong>
        {successRate}%
    </div>
</section>

<hr />

<section>
    <h2>Active grants</h2>

    {#if inProgressGrants.length === 0}
        <p>No active grants.</p>
    {:else}
        {#each inProgressGrants as grant}
            <article>
                <strong>{grant.name}</strong>
                <div>{grant.funder}</div>
                <div>{grant.deadline}</div>
                <div>{grant.status}</div>
            </article>
        {/each}
    {/if}

    <a href="/grants">
        View all grants
    </a>
</section>

<hr />

<section>
    <h2>Manuscripts</h2>

    {#if manuscripts.length === 0}
        <p>No manuscripts.</p>
    {:else}
        {#each manuscripts as manuscript}
            <article>
                <strong>{manuscript.title}</strong>
                <div>{manuscript.journal}</div>
                <div>{manuscript.status}</div>
                <div>{manuscript.nextAction}</div>
            </article>
        {/each}
    {/if}

    <a href="/manuscripts">
        View all manuscripts
    </a>
</section>