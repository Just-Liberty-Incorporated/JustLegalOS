<script lang="ts">
  import DraftToPdfCard from '$lib/components/DraftToPdfCard.svelte';
  let markdown = $state('');
</script>
<main class="page-content">
  <header class="page-header"><div><h1>Draft formatter</h1><p>Prepare your own Markdown draft, review the preview, and export Word and PDF.</p></div></header>
  <DraftToPdfCard bind:markdown />
</main>
