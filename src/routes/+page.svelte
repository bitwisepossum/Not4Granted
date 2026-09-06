<script lang="ts">
  import "../app.css";

  let { children } = $props();

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
      status: "Rejected",
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
      <ManuscriptsView {manuscripts} onNavigate={(view) => activeView = view} />
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