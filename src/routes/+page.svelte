<script lang="ts">
  import "../app.css";
  import { addGrant } from "../components/api";

  let { children } = $props();

  import type { Grant, NewGrant, Manuscript, View } from "../types";
  import DashboardView from "../views/DashboardView.svelte";
  import GrantsView from "../views/GrantsView.svelte";
  import ManuscriptsView from "../views/ManuscriptsView.svelte";
  import AddGrantView from "../views/AddGrantView.svelte";
  import AddManuscriptView from "../views/AddManuscriptView.svelte";

  let activeView = $state<View>("dashboard");

  let grants = $state<Grant[]>([
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
  ]);

  let manuscripts = $state<Manuscript[]>([
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
  ]);

  async function handleGrantSubmit(grant: NewGrant) {
    const savedGrant = await addGrant(grant);

    grants = [...grants, savedGrant];
    activeView = "grants";
  }
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

    <div class="sidebar-footer">local database target</div>
  </aside>

  <main>
    <header>
      <div>
        <h1>{activeView === "dashboard" ? "Dashboard" : activeView === "grants" ? "Grants" : "Manuscripts"}</h1>
      </div>
      <button onclick={() => activeView = "add-grant"}>+ Add Grant</button>
      <button onclick={() => activeView = "add-manuscript"}>+ Add Manuscript</button>
    </header>

    {#if activeView === "dashboard"}
      <DashboardView {grants} {manuscripts} onNavigate={(view) => activeView = view} />
    {:else if activeView === "grants"}
      <GrantsView {grants} onNavigate={(view) => activeView = view} />
    {:else if activeView === "add-grant"}
      <AddGrantView
        onCancel={() => activeView = "grants"}
        onSubmit={handleGrantSubmit}
      />
    {:else if activeView === "manuscripts"}
      <ManuscriptsView {manuscripts} onNavigate={(view) => activeView = view} />
    {:else if activeView === "add-manuscript"}
      <AddManuscriptView
        onCancel={() => activeView = "manuscripts"}
        onSubmit={(manuscript) => {
          manuscripts = [...manuscripts, manuscript];
          activeView = "manuscripts";
        }}
      />
    {:else}
      <p>Unknown view: {activeView}</p>
    {/if}
  </main>
</div>