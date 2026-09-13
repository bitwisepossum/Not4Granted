<script lang="ts">
    import "../styles/dashboard.css";
    import type { Grant, Manuscript } from "../types";

    let {
        grants,
        manuscripts,
        active_grants,
        active_manuscripts,
        isLoading,
        error
    }: {
        grants: Grant[];
        manuscripts: Manuscript[];
        active_grants: Grant[];
        active_manuscripts: Manuscript[];
        isLoading: boolean;
        error?: string;
    } = $props();

    const appliedGrants = $derived(
        grants.filter((grant) => grant.status !== "Planning")
    );

    const acceptedGrants = $derived(
        grants.filter((grant) => grant.status === "Accepted")
    );

    const rejectedGrants = $derived(
        grants.filter((grant) => grant.status === "Rejected")
    );

    const planningGrants = $derived(
        grants.filter((grant) => grant.status === "Planning")
    );

    const submittedGrants = $derived(
        grants.filter((grant) => grant.status === "Submitted")
    );

    const decidedGrants = $derived(
        grants.filter(
            (grant) =>
                grant.status === "Accepted" ||
                grant.status === "Rejected"
        )
    );

    const successRate = $derived(
        decidedGrants.length === 0
            ? 0
            : Math.round(
                  (acceptedGrants.length / decidedGrants.length) * 100
              )
    );

    const fundingTotals = $derived.by(() => {
        const totals = new Map<
            string,
            { requested: number; granted: number }
        >();

        for (const grant of grants) {
            const currency = grant.currency || "EUR";
            const current = totals.get(currency) ?? {
                requested: 0,
                granted: 0
            };

            current.requested += grant.amountRequested ?? 0;
            current.granted += grant.amountReceived ?? 0;
            totals.set(currency, current);
        }

        return [...totals.entries()]
            .map(([currency, amounts]) => ({ currency, ...amounts }))
            .sort((a, b) => a.currency.localeCompare(b.currency));
    });

    const submittedManuscripts = $derived(
        manuscripts.filter((manuscript) => manuscript.status === "Submitted")
    );

    const rejectedManuscripts = $derived(
        manuscripts.filter((manuscript) => manuscript.status === "Rejected")
    );

    const acceptedOrPublishedManuscripts = $derived(
        manuscripts.filter(
            (manuscript) =>
                manuscript.status === "Accepted" ||
                manuscript.status === "Published"
        )
    );

    function formatAmount(amount: number, currency: string): string {
        return new Intl.NumberFormat(undefined, {
            style: "currency",
            currency,
            maximumFractionDigits: 0
        }).format(amount / 100);
    }

    function statusClass(status: string): string {
        return status.toLowerCase();
    }

    function grantBarWidth(count: number): number {
        if (grants.length === 0) return 0;
        return (count / grants.length) * 100;
    }
</script>

<section class="dashboard">
    <header class="page-header">
        <div>
            <h1>Dashboard</h1>
            <p>Current grant and manuscript activity.</p>
        </div>
    </header>

    {#if isLoading}
        <div class="empty-state">
            <h2>Loading dashboard</h2>
            <p>Consulting the database about the current state of academic optimism.</p>
        </div>
    {:else if error}
        <div class="empty-state">
            <h2>Dashboard unavailable</h2>
            <p>{error}</p>
        </div>
    {:else}
        <div class="dashboard-lists">
            <section class="panel dashboard-list-panel">
                <div class="panel-heading">
                    <div>
                        <h2>Active grants</h2>
                        <p>Currently {active_grants.length} grant applications are pending.</p>
                    </div>

                    <a class="button-link" href="/grants">
                        View all
                    </a>
                </div>

                {#if active_grants.length === 0}
                    <div class="empty-state">
                        <h2>No active grants</h2>
                        <p>Nothing is currently awaiting either paperwork or judgment.</p>
                    </div>
                {:else}
                    {#each active_grants as grant}
                        <a
                            class="list-row dashboard-list-row"
                            href={`/grants/${grant.id}`}
                        >
                            <div class="dashboard-list-primary">
                                <strong>{grant.name}</strong>
                                <span>{grant.funder}</span>
                            </div>

                            <div class="dashboard-list-meta">
                                <span>{grant.deadline ?? "No deadline"}</span>
                                <span class={`status ${statusClass(grant.status)}`}>
                                    {grant.status}
                                </span>
                            </div>
                        </a>
                    {/each}
                {/if}
            </section>

            <section class="panel dashboard-list-panel">
                <div class="panel-heading">
                    <div>
                        <h2>Active manuscripts</h2>
                        <p>Currently {active_manuscripts.length} manuscripts are in progress.</p>
                    </div>

                    <a class="button-link" href="/manuscripts">
                        View all
                    </a>
                </div>

                {#if active_manuscripts.length === 0}
                    <div class="empty-state">
                        <h2>No active manuscripts</h2>
                        <p>An unusual and potentially medically significant calm.</p>
                    </div>
                {:else}
                    {#each active_manuscripts as manuscript}
                        <a
                            class="list-row dashboard-list-row"
                            href={`/manuscripts/${manuscript.id}`}
                        >
                            <div class="dashboard-list-primary">
                                <strong class="dashboard-manuscript-title">
                                    {manuscript.title}
                                </strong>
                                <span>{manuscript.journal ?? "No journal selected"}</span>
                            </div>

                            <div class="dashboard-list-meta">
                                <span>{manuscript.nextAction ?? "No next action"}</span>
                                <span class={`status ${statusClass(manuscript.status)}`}>
                                    {manuscript.status}
                                </span>
                            </div>
                        </a>
                    {/each}
                {/if}
            </section>
        </div>

        <header class="page-subheader">
            <div>
                <h2>Grant statistics</h2>
            </div>
        </header>

        <div class="dashboard-stats">
            <article>
                <span>Applied</span>
                <strong>{appliedGrants.length}</strong>
                <small>Submitted or decided applications</small>
            </article>

            <article>
                <span>Accepted</span>
                <strong>{acceptedGrants.length}</strong>
                <small>Applications receiving funding</small>
            </article>

            <article>
                <span>Rejected</span>
                <strong>{rejectedGrants.length}</strong>
                <small>Applications contributing to resilience</small>
            </article>
        </div>

        <div class="dashboard-grid">
            <section class="panel dashboard-chart-panel">
                <div class="panel-heading">
                    <div>
                        <h2>Grant status</h2>
                        <p>Distribution across all tracked grants.</p>
                    </div>
                </div>

                <div class="dashboard-bar-chart">
                    <div class="dashboard-bar-row">
                        <span>Planning</span>
                        <div class="dashboard-bar-track">
                            <div
                                class="dashboard-bar-fill in-progress"
                                style={`width: ${grantBarWidth(planningGrants.length)}%`}
                            ></div>
                        </div>
                        <strong>{planningGrants.length}</strong>
                    </div>

                    <div class="dashboard-bar-row">
                        <span>Submitted</span>
                        <div class="dashboard-bar-track">
                            <div
                                class="dashboard-bar-fill in-progress"
                                style={`width: ${grantBarWidth(submittedGrants.length)}%`}
                            ></div>
                        </div>
                        <strong>{submittedGrants.length}</strong>
                    </div>

                    <div class="dashboard-bar-row">
                        <span>Accepted</span>
                        <div class="dashboard-bar-track">
                            <div
                                class="dashboard-bar-fill accepted"
                                style={`width: ${grantBarWidth(acceptedGrants.length)}%`}
                            ></div>
                        </div>
                        <strong>{acceptedGrants.length}</strong>
                    </div>

                    <div class="dashboard-bar-row">
                        <span>Rejected</span>
                        <div class="dashboard-bar-track">
                            <div
                                class="dashboard-bar-fill rejected"
                                style={`width: ${grantBarWidth(rejectedGrants.length)}%`}
                            ></div>
                        </div>
                        <strong>{rejectedGrants.length}</strong>
                    </div>
                </div>

                <div class="funding-ledger">
                    {#if fundingTotals.length === 0}
                        <p class="outcome-empty">No funding amounts recorded.</p>
                    {:else}
                        {#each fundingTotals as total}
                            <div class="funding-row">
                                <div>
                                    <span>Requested</span>
                                    <strong>{formatAmount(total.requested, total.currency)}</strong>
                                </div>

                                <div>
                                    <span>Awarded</span>
                                    <strong>{formatAmount(total.granted, total.currency)}</strong>
                                </div>
                            </div>
                        {/each}
                    {/if}
                </div>
            </section>

            <section class="panel dashboard-success-panel">
                <div class="panel-heading">
                    <div>
                        <h2>Funding success</h2>
                        <p>Based on decided applications.</p>
                    </div>
                </div>

                <div
                    class="dashboard-donut"
                    style={`--value: ${successRate * 3.6}deg`}
                >
                    <div>
                        <strong>{successRate}%</strong>
                        <span>success rate</span>
                    </div>
                </div>

                <div class="dashboard-decision-breakdown">
                    <span>
                        Accepted
                        <strong>{acceptedGrants.length}</strong>
                    </span>

                    <span>
                        Rejected
                        <strong>{rejectedGrants.length}</strong>
                    </span>
                </div>
            </section>
        </div>

        <header class="page-subheader">
            <div>
                <h2>Manuscript outcomes</h2>
            </div>
        </header>

        <section class="panel dashboard-manuscript-outcomes">
            <div class="panel-heading">
                <div>
                    <h2>Current publication status across tracked manuscripts.</h2>
                </div>
            </div>

            <div class="manuscript-judgment">
                <div class="judgment-counts">
                    <div>
                        <strong>{submittedManuscripts.length}</strong>
                        <span>Submitted</span>
                        <small>Pending external assessment</small>
                    </div>

                    <div class="rejected-count">
                        <strong>{rejectedManuscripts.length}</strong>
                        <span>Rejected</span>
                        <small>Professional development</small>
                    </div>

                    <div class="accepted-count">
                        <strong>{acceptedOrPublishedManuscripts.length}</strong>
                        <span>Accepted + published</span>
                        <small>Converted to research output</small>
                    </div>
                </div>
            </div>
        </section>

    {/if}
</section>