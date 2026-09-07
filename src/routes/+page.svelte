<script lang="ts">
  import { onMount } from "svelte";
  import "../app.css";
  import DashboardView from "../views/DashboardView.svelte";
  import { getGrants, getManuscripts } from "../components/api";
  import type { Grant, Manuscript } from "../types";

  let grants = $state<Grant[]>([]);
  let manuscripts = $state<Manuscript[]>([]);

  onMount(async () => {
    grants = await getGrants();
    manuscripts = await getManuscripts();
  });
</script>

<svelte:head>
  <title>Not4Granted</title>
</svelte:head>

<DashboardView {grants} {manuscripts} />