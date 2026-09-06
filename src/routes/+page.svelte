<script lang="ts">
  type Grant = {
    id: number;
    name: string;
    funder: string;
    deadline: string;
    amount: string;
    status: "Planning" | "Submitted" | "Awarded" | "Rejected";
  };

  type Manuscript = {
    id: number;
    title: string;
    journal: string;
    status: "Drafting" | "Submitted" | "Revision" | "Accepted";
    nextAction: string;
  };

  let activeView = $state<"dashboard" | "grants" | "manuscripts">("dashboard");

  
  const grants: Grant[] = [
    {
      id: 1,
      name: "Doctoral research grant",
      funder: "Example Foundation",
      deadline: "2026-10-15",
      amount: "€24,000",
      status: "Planning"
    },
    {
      id: 2,
      name: "Conference travel grant",
      funder: "Research Society",
      deadline: "2026-09-30",
      amount: "€1,500",
      status: "Submitted"
    }
  ];

  const manuscripts: Manuscript[] = [
    {
      id: 1,
      title: "Minority stress in healthcare",
      journal: "BMC Nursing",
      status: "Drafting",
      nextAction: "Finish results section"
    },
    {
      id: 2,
      title: "Integrative review",
      journal: "Advances in Nursing Science",
      status: "Accepted",
      nextAction: "Check proofs"
    }
  ];

  const statusClass = (status: string) => status.toLowerCase().replace(" ", "-");
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

    <div class="sidebar-footer">Local database</div>
  </aside>

  <main>
    <header>
      <div>
        <h1>{activeView === "dashboard" ? "Dashboard" : activeView === "grants" ? "Grants" : "Manuscripts"}</h1>
        <p>Funding applications and publication work in one mildly less chaotic place.</p>
      </div>
      <button class="primary">+ Add item</button>
    </header>

    {#if activeView === "dashboard"}
      <section class="stats">
        <article>
          <span>Active grants</span>
          <strong>{grants.length}</strong>
        </article>
        <article>
          <span>Submitted</span>
          <strong>{grants.filter((grant) => grant.status === "Submitted").length}</strong>
        </article>
        <article>
          <span>Manuscripts</span>
          <strong>{manuscripts.length}</strong>
        </article>
        <article>
          <span>Accepted</span>
          <strong>{manuscripts.filter((manuscript) => manuscript.status === "Accepted").length}</strong>
        </article>
      </section>

      <section class="grid">
        <div class="panel">
          <div class="panel-heading">
            <h2>Upcoming grants</h2>
            <button class="link-button" onclick={() => activeView = "grants"}>View all</button>
          </div>

          {#each grants as grant}
            <button class="list-row">
              <div>
                <strong>{grant.name}</strong>
                <span>{grant.funder}</span>
              </div>
              <div class="row-meta">
                <span>{grant.deadline}</span>
                <span class="status {statusClass(grant.status)}">{grant.status}</span>
              </div>
            </button>
          {/each}
        </div>

        <div class="panel">
          <div class="panel-heading">
            <h2>Manuscript work</h2>
            <button class="link-button" onclick={() => activeView = "manuscripts"}>View all</button>
          </div>

          {#each manuscripts as manuscript}
            <button class="list-row">
              <div>
                <strong>{manuscript.title}</strong>
                <span>{manuscript.nextAction}</span>
              </div>
              <span class="status {statusClass(manuscript.status)}">{manuscript.status}</span>
            </button>
          {/each}
        </div>
      </section>
    {:else if activeView === "grants"}
      <section class="panel table-panel">
        <div class="panel-heading">
          <h2>Grant applications</h2>
          <input placeholder="Search grants..." />
        </div>

        <div class="table">
          <div class="table-row table-header">
            <span>Name</span>
            <span>Funder</span>
            <span>Deadline</span>
            <span>Amount</span>
            <span>Status</span>
          </div>
          {#each grants as grant}
            <button class="table-row">
              <strong>{grant.name}</strong>
              <span>{grant.funder}</span>
              <span>{grant.deadline}</span>
              <span>{grant.amount}</span>
              <span class="status {statusClass(grant.status)}">{grant.status}</span>
            </button>
          {/each}
        </div>
      </section>
    {:else}
      <section class="panel table-panel">
        <div class="panel-heading">
          <h2>Manuscripts</h2>
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
    font-family: Inter, system-ui, sans-serif;
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
    background: #d8dee8;
    color: #20242a;
  }

  .brand strong,
  .brand span {
    display: block;
  }

  .brand span {
    margin-top: 2px;
    color: #979faa;
    font-size: 12px;
  }

  nav {
    display: grid;
    gap: 4px;
  }

  nav button {
    padding: 10px 12px;
    border: 0;
    border-radius: 7px;
    text-align: left;
    background: transparent;
    color: #b9c0c9;
    cursor: pointer;
  }

  nav button:hover,
  nav button.active {
    color: #fff;
    background: #30363e;
  }

  .sidebar-footer {
    margin-top: auto;
    padding: 10px 8px 0;
    color: #737c87;
    font-size: 12px;
  }

  main {
    padding: 32px;
    overflow: auto;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
    margin-bottom: 28px;
  }

  h1,
  h2,
  p {
    margin: 0;
  }

  h1 {
    font-size: 26px;
  }

  h2 {
    font-size: 16px;
  }

  header p {
    margin-top: 5px;
    color: #737b86;
    font-size: 14px;
  }

  .primary {
    padding: 9px 14px;
    border: 0;
    border-radius: 7px;
    color: white;
    background: #303842;
    cursor: pointer;
  }

  .primary:hover {
    background: #1f252c;
  }

  .stats {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 14px;
    margin-bottom: 18px;
  }

  .stats article,
  .panel {
    border: 1px solid #e0e3e7;
    border-radius: 10px;
    background: white;
  }

  .stats article {
    padding: 18px;
  }

  .stats span {
    display: block;
    margin-bottom: 6px;
    color: #727a84;
    font-size: 12px;
  }

  .stats strong {
    font-size: 24px;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 18px;
  }

  .panel {
    overflow: hidden;
  }

  .panel-heading {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 16px 18px;
    border-bottom: 1px solid #eceef1;
  }

  .panel-heading input {
    width: 220px;
    padding: 7px 10px;
    border: 1px solid #d7dbe0;
    border-radius: 6px;
    outline: none;
  }

  .panel-heading input:focus {
    border-color: #8d98a5;
  }

  .link-button {
    border: 0;
    background: transparent;
    color: #596574;
    font-size: 12px;
    cursor: pointer;
  }

  .list-row {
    display: flex;
    width: 100%;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 14px 18px;
    border: 0;
    border-bottom: 1px solid #f0f1f3;
    background: white;
    text-align: left;
    cursor: pointer;
  }

  .list-row:last-child {
    border-bottom: 0;
  }

  .list-row:hover,
  .table-row:not(.table-header):hover {
    background: #f8f9fa;
  }

  .list-row strong,
  .list-row span {
    display: block;
  }

  .list-row strong {
    font-size: 13px;
  }

  .list-row div > span {
    margin-top: 3px;
    color: #7d858f;
    font-size: 12px;
  }

  .row-meta {
    text-align: right;
  }

  .row-meta > span:first-child {
    margin-bottom: 4px;
  }

  .status {
    display: inline-block;
    width: fit-content;
    padding: 3px 7px;
    border-radius: 999px;
    color: #58616b;
    background: #edf0f3;
    font-size: 11px;
    font-weight: 600;
  }

  .status.submitted,
  .status.revision {
    color: #795b14;
    background: #fff2c9;
  }

  .status.awarded,
  .status.accepted {
    color: #27613e;
    background: #dff3e7;
  }

  .status.rejected {
    color: #843b3b;
    background: #f7dfdf;
  }

  .table-panel {
    overflow-x: auto;
  }

  .table {
    min-width: 760px;
  }

  .table-row {
    display: grid;
    width: 100%;
    grid-template-columns: 1.5fr 1fr 120px 100px 100px;
    align-items: center;
    gap: 16px;
    padding: 12px 18px;
    border: 0;
    border-bottom: 1px solid #eceef1;
    background: white;
    text-align: left;
  }

  button.table-row {
    cursor: pointer;
  }

  .table-row:last-child {
    border-bottom: 0;
  }

  .table-header {
    color: #7c848e;
    background: #fafbfc;
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
  }

  .manuscript-table .table-row {
    grid-template-columns: 1.5fr 1fr 100px 1.2fr;
  }

  .table-row strong,
  .table-row span {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 13px;
  }

  @media (max-width: 900px) {
    .stats {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .grid {
      grid-template-columns: 1fr;
    }
  }
</style>