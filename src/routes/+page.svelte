<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { addGrant, getGrants, addManuscript, getManuscripts } from "../components/api";
  import type { Grant, NewGrant, Manuscript, View } from "../types";
  import DashboardView from "../views/DashboardView.svelte";
  import GrantsView from "../views/GrantsView.svelte";
  import ManuscriptsView from "../views/ManuscriptsView.svelte";
  import AddGrantView from "../views/AddGrantView.svelte";
  import AddManuscriptView from "../views/AddManuscriptView.svelte";

  let { children } = $props();

  onMount(async () => {
    grants = await getGrants();
    manuscripts = await getManuscripts();
  });

  let activeView = $state<View>("dashboard");

  let grants = $state<Grant[]>([]);
  let manuscripts = $state<Manuscript[]>([]);

  // Handle submission of a new grant
  async function handleGrantSubmit(grant: NewGrant) {
    const savedGrant = await addGrant(grant);

    grants = [...grants, savedGrant];
    activeView = "grants";
  }

  // Handle submission of a new manuscript
  async function handleManuscriptSubmit(manuscript: NewManuscript) {
    const savedManuscript = await addManuscript(manuscript);

    manuscripts = [...manuscripts, savedManuscript];
    activeView = "manuscripts";
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
        onSubmit={handleManuscriptSubmit}
      />
    {:else}
      <p>Unknown view: {activeView}</p>
    {/if}
  </main>
</div>