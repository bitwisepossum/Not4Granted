<script lang="ts">
  import type { Grant, Manuscript, View } from "../types";
  import DashboardView from "../views/DashboardView.svelte";
  import GrantsView from "../views/GrantsView.svelte";
  import ManuscriptsView from "../views/ManuscriptsView.svelte";
  import AddGrantView from "../views/AddGrantView.svelte";
  import AddManuscriptView from "../views/AddManuscriptView.svelte";


  let activeView = $state<View>("dashboard");

  const grants: Grant[] = [
    {
      id: 1,
      name: "Urban Pollinator Microgrant",
      funder: "Northbridge Research Trust",
      deadline: "2026-09-28",
      amount: "€8,500",
      status: "Planning"
    },
    {
      id: 2,
      name: "Open Tools Seed Funding",
      funder: "Example Science Foundation",
      deadline: "2026-10-12",
      amount: "€15,000",
      status: "Submitted"
    },
    {
      id: 3,
      name: "Early Career Mobility Grant",
      funder: "Baltic Academic Council",
      deadline: "2026-11-05",
      amount: "€3,200",
      status: "Submitted"
    },
    {
      id: 4,
      name: "Community Data Pilot",
      funder: "Civic Knowledge Fund",
      deadline: "2026-05-14",
      amount: "€21,000",
      status: "Accepted"
    },
    {
      id: 5,
      name: "Small Methods Development Grant",
      funder: "Helix Research Society",
      deadline: "2026-03-31",
      amount: "€12,000",
      status: "Rejected"
    },
    {
      id: 6,
      name: "Research Software Support Award",
      funder: "Fictional Open Research Fund",
      deadline: "2025-12-15",
      amount: "€9,500",
      status: "Rejected"
    },
    {
      id: 7,
      name: "Interdisciplinary Workshop Fund",
      funder: "Example University Network",
      deadline: "2026-01-20",
      amount: "€4,000",
      status: "Rejected"
    },
    {
      id: 8,
      name: "Prototype Evaluation Grant",
      funder: "North Coast Innovation Fund",
      deadline: "2026-12-01",
      amount: "€18,000",
      status: "Planning"
    },
    {
      id: 9,
      name: "Conference Participation Award",
      funder: "International Methods Association",
      deadline: "2026-08-30",
      amount: "€1,800",
      status: "Rejected"
    },
    {
      id: 10,
      name: "Exploratory Research Award",
      funder: "Mock Funding Council",
      deadline: "2026-02-18",
      amount: "€25,000",
      status: "Rejected"
    },
    {
      id: 11,
      name: "Open Scholarship Infrastructure Grant",
      funder: "Fictional Research Infrastructure Fund",
      deadline: "2026-04-22",
      amount: "€32,000",
      status: "Rejected"
    },
    {
      id: 12,
      name: "Research Exchange Travel Award",
      funder: "Northern Academic Exchange",
      deadline: "2026-06-10",
      amount: "€2,700",
      status: "Rejected"
    },
    {
      id: 13,
      name: "Small Dataset Reuse Award",
      funder: "Example Data Science Trust",
      deadline: "2026-07-07",
      amount: "€6,000",
      status: "Rejected"
    },
    {
      id: 14,
      name: "Methods Training Support Grant",
      funder: "Demo Academic Development Fund",
      deadline: "2026-11-30",
      amount: "€5,500",
      status: "Submitted"
    }
  ];

  const manuscripts: Manuscript[] = [
    {
      id: 1,
      title: "A Small Study of Very Serious Spreadsheet Problems",
      journal: "Journal of Demonstrational Research",
      status: "Drafting",
      nextAction: "Finish methods draft"
    },
    {
      id: 2,
      title: "Local-First Research Tools in Small Academic Teams",
      journal: "Open Methods Quarterly",
      status: "Submitted",
      nextAction: "Wait for editor decision"
    },
    {
      id: 3,
      title: "Why Researchers Keep Inventing Their Own Trackers",
      journal: "Academic Workflow Review",
      status: "Revision",
      nextAction: "Address reviewer 2"
    },
    {
      id: 4,
      title: "Metadata Practices in Fictional Field Studies",
      journal: "Data Practice Notes",
      status: "Accepted",
      nextAction: "Check proofs"
    },
    {
      id: 5,
      title: "A Pilot Taxonomy of Deadline-Induced Panic",
      journal: "Journal of Entirely Plausible Studies",
      status: "Idea",
      nextAction: "Write outline"
    }
  ];

  const appliedGrants = grants.filter((grant) => grant.status !== "Planning");
  const acceptedGrants = grants.filter((grant) => grant.status === "Accepted");
  const rejectedGrants = grants.filter((grant) => grant.status === "Rejected");
  const decidedGrants = grants.filter(
    (grant) => grant.status === "Accepted" || grant.status === "Rejected"
  );
  const inProgressGrants = grants.filter(
    (grant) => grant.status === "Planning" || grant.status === "Submitted"
  );

  const grantStats = [
    { label: "Applied", value: appliedGrants.length },
    { label: "Accepted", value: acceptedGrants.length },
    { label: "Rejected", value: rejectedGrants.length },
    { label: "In progress", value: inProgressGrants.length }
  ];

  const maxGrantStat = Math.max(...grantStats.map((stat) => stat.value), 1);
  const successRate = decidedGrants.length === 0
    ? 0
    : Math.round((acceptedGrants.length / decidedGrants.length) * 100);

  const statusClass = (status: string) => status.toLowerCase().replaceAll(" ", "-");
</script>

<svelte:head>
  <title>Not4Granted</title>
</svelte:head>

<div class="app-shell">
  <aside class="sidebar">
    <div class="brand">
      <div class="brand-mark">N4G</div>
      <div>
        <strong>Not4Granted</strong>
        <span>Research tracker</span>
      </div>
    </div>

    <nav>
      <button class:active={activeView === "dashboard"} onclick={() => activeView = "dashboard"}>
        Dashboard
      </button>
      <button class:active={activeView === "grants"} onclick={() => activeView = "grants"}>
        Grants
      </button>
      <button class:active={activeView === "manuscripts"} onclick={() => activeView = "manuscripts"}>
        Manuscripts
      </button>
    </nav>

    <div class="sidebar-footer">Demo data · local database target</div>
  </aside>

  <main>
    <header>
      <div>
        <h1>{activeView === "dashboard" ? "Dashboard" : activeView === "grants" ? "Grants" : "Manuscripts"}</h1>
        <p>Demonstrational data only. No dissertation, funder, journal or manuscript here is meant to represent real project data.</p>
      </div>
      <button onclick={() => activeView = "add-grant"}>+ Add Grant</button>
      <button onclick={() => activeView = "add-manuscript"}>+ Add Manuscript</button>
    </header>

    {#if activeView === "dashboard"}
      <DashboardView {grants} {manuscripts} onNavigate={(view) => activeView = view} />
    {:else if activeView === "grants"}
      <GrantsView {grants} onNavigate={(view) => activeView = view} />
    {:else if activeView === "add-grant"}
      <AddGrantView {grants} onNavigate={(view) => activeView = view} />
    {:else if activeView === "manuscripts"}
      <ManuscriptsView {manuscripts} />
    {:else if activeView === "add-manuscript"}
      <AddManuscriptView {manuscripts} onNavigate={(view) => activeView = view} />
    {:else}
      <section class="panel table-panel">
        <div class="panel-heading">
          <div>
            <h2>Manuscripts</h2>
            <p>Entirely fictional manuscript records</p>
          </div>
          <input placeholder="Search manuscripts..." />
        </div>

        <div class="table manuscript-table">
          <div class="table-row table-header">
            <span>Title</span>
            <span>Journal</span>
            <span>Status</span>
            <span>Next action</span>
          </div>
          {#each manuscripts as manuscript}
            <button class="table-row">
              <strong>{manuscript.title}</strong>
              <span>{manuscript.journal}</span>
              <span class="status {statusClass(manuscript.status)}">{manuscript.status}</span>
              <span>{manuscript.nextAction}</span>
            </button>
          {/each}
        </div>
      </section>
    {/if}
  </main>
</div>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    font-family: Inter, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    color: #20242a;
    background: #f5f6f8;
  }

  button,
  input {
    font: inherit;
  }

  button {
    color: inherit;
  }

  .app-shell {
    display: grid;
    grid-template-columns: 220px 1fr;
    min-height: 100vh;
  }

  .sidebar {
    display: flex;
    flex-direction: column;
    padding: 20px 14px;
    color: #e8ebef;
    background: #20242a;
  }

  .brand {
    display: flex;
    gap: 10px;
    align-items: center;
    padding: 4px 8px 22px;
  }

  .brand-mark {
    display: grid;
    width: 38px;
    height: 38px;
    place-items: center;
    border-radius: 8px;
    font-size: 12px;
    font-weight: 800;
    color: #20242a;
    background: #d8dee8;
  }

  .brand strong,
  .brand span {
    display: block;
  }

  .brand span {
    margin-top: 2px;
    color: #98a1ad;
    font-size: 12px;
  }

  nav {
    display: grid;
    gap: 5px;
  }

  nav button {
    padding: 10px 12px;
    border: 0;
    border-radius: 7px;
    text-align: left;
    color: #b9c0ca;
    background: transparent;
    cursor: pointer;
  }

  nav button:hover,
  nav button.active {
    color: white;
    background: #30363e;
  }

  .sidebar-footer {
    margin-top: auto;
    padding: 12px 8px 2px;
    color: #7e8792;
    font-size: 11px;
  }

  main {
    min-width: 0;
    padding: 30px;
  }

  header {
    display: flex;
    justify-content: space-between;
    gap: 24px;
    align-items: flex-start;
    margin-bottom: 26px;
  }

  h1,
  h2,
  p {
    margin: 0;
  }

  h1 {
    font-size: 28px;
  }

  h2 {
    font-size: 16px;
  }

  header p,
  .panel-heading p {
    margin-top: 5px;
    color: #747d89;
    font-size: 13px;
  }

  .primary {
    padding: 9px 14px;
    border: 0;
    border-radius: 7px;
    font-weight: 650;
    color: white;
    background: #313b49;
    cursor: pointer;
  }

  .stats {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 14px;
    margin-bottom: 16px;
  }

  .stats article,
  .panel {
    border: 1px solid #dde1e6;
    border-radius: 10px;
    background: white;
  }

  .stats article {
    padding: 17px;
  }

  .stats span,
  .stats small,
  .stats strong {
    display: block;
  }

  .stats span {
    color: #68717d;
    font-size: 12px;
    font-weight: 650;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .stats strong {
    margin: 6px 0 2px;
    font-size: 27px;
  }

  .stats small {
    color: #9299a3;
    font-size: 11px;
  }

  .grid {
    display: grid;
    grid-template-columns: minmax(0, 1.2fr) minmax(0, 1fr);
    gap: 16px;
    margin-bottom: 16px;
  }

  .dashboard-grid {
    grid-template-columns: minmax(0, 1.5fr) minmax(260px, 0.7fr);
  }

  .panel {
    overflow: hidden;
  }

  .panel-heading {
    display: flex;
    justify-content: space-between;
    gap: 18px;
    align-items: center;
    padding: 16px 18px;
    border-bottom: 1px solid #e8ebef;
  }

  .link-button {
    border: 0;
    color: #5b6571;
    background: transparent;
    cursor: pointer;
  }

  .list-row {
    display: flex;
    width: 100%;
    justify-content: space-between;
    gap: 20px;
    align-items: center;
    padding: 14px 18px;
    border: 0;
    border-bottom: 1px solid #edf0f2;
    text-align: left;
    background: white;
    cursor: pointer;
  }

  .list-row:last-child {
    border-bottom: 0;
  }

  .list-row:hover,
  .table-row:not(.table-header):hover {
    background: #fafbfc;
  }

  .list-row strong,
  .list-row span {
    display: block;
  }

  .list-row strong {
    font-size: 13px;
  }

  .list-row div > span {
    margin-top: 4px;
    color: #7d8590;
    font-size: 12px;
  }

  .row-meta {
    min-width: 125px;
    text-align: right;
  }

  .row-meta .status {
    display: inline-block;
    margin-top: 5px;
  }

  .chart-panel,
  .success-panel {
    min-height: 280px;
  }

  .bar-chart {
    display: grid;
    gap: 17px;
    padding: 24px 20px 26px;
  }

  .bar-row {
    display: grid;
    grid-template-columns: 80px minmax(0, 1fr) 28px;
    gap: 12px;
    align-items: center;
    font-size: 12px;
  }

  .bar-row > span {
    color: #636d78;
  }

  .bar-row > strong {
    text-align: right;
    font-size: 12px;
  }

  .bar-track {
    height: 12px;
    overflow: hidden;
    border-radius: 999px;
    background: #edf0f3;
  }

  .bar-fill {
    height: 100%;
    min-width: 4px;
    border-radius: inherit;
    background: #697584;
  }

  .bar-fill.accepted {
    background: #5c8067;
  }

  .bar-fill.rejected {
    background: #8b6363;
  }

  .bar-fill.in-progress {
    background: #7a718e;
  }

  .success-panel {
    display: flex;
    flex-direction: column;
  }

  .donut {
    --value: 0deg;
    display: grid;
    width: 150px;
    height: 150px;
    place-items: center;
    align-self: center;
    margin: 24px 0 16px;
    border-radius: 50%;
    background: conic-gradient(#5c8067 var(--value), #e8ecef 0deg);
  }

  .donut::before {
    content: "";
    grid-area: 1 / 1;
    width: 108px;
    height: 108px;
    border-radius: 50%;
    background: white;
  }

  .donut > div {
    z-index: 1;
    grid-area: 1 / 1;
    text-align: center;
  }

  .donut strong,
  .donut span {
    display: block;
  }

  .donut strong {
    font-size: 25px;
  }

  .donut span {
    margin-top: 2px;
    color: #7a838e;
    font-size: 11px;
  }

  .decision-breakdown {
    display: flex;
    justify-content: center;
    gap: 18px;
    padding: 0 18px 22px;
    color: #747d89;
    font-size: 11px;
  }

  .decision-breakdown strong {
    color: #30363e;
  }

  .table-panel {
    width: 100%;
  }

  .panel-heading input {
    width: 220px;
    padding: 8px 10px;
    border: 1px solid #d9dee4;
    border-radius: 7px;
    outline: none;
  }

  .table {
    overflow-x: auto;
  }

  .table-row {
    display: grid;
    grid-template-columns: minmax(220px, 2fr) minmax(170px, 1.2fr) 115px 100px 110px;
    width: 100%;
    min-width: 820px;
    gap: 14px;
    align-items: center;
    padding: 13px 18px;
    border: 0;
    border-bottom: 1px solid #edf0f2;
    text-align: left;
    background: white;
  }

  button.table-row {
    cursor: pointer;
  }

  .table-header {
    color: #858d97;
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    background: #fafbfc;
  }

  .table-row:not(.table-header) {
    font-size: 12px;
  }

  .manuscript-table .table-row {
    grid-template-columns: minmax(280px, 2fr) minmax(190px, 1.2fr) 110px minmax(180px, 1.2fr);
  }

  .status {
    width: fit-content;
    padding: 4px 7px;
    border-radius: 999px;
    color: #5f6874;
    font-size: 10px;
    font-weight: 700;
    background: #eceff2;
  }

  .status.accepted {
    color: #42634d;
    background: #e5eee8;
  }

  .status.rejected {
    color: #714d4d;
    background: #f1e6e6;
  }

  .status.submitted,
  .status.revision {
    color: #685f42;
    background: #f1eddf;
  }

  .status.planning,
  .status.drafting,
  .status.idea {
    color: #565f70;
    background: #e8ebf0;
  }

  @media (max-width: 950px) {
    .stats {
      grid-template-columns: repeat(2, 1fr);
    }

    .grid,
    .dashboard-grid {
      grid-template-columns: 1fr;
    }
  }
</style>