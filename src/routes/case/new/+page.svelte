<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { ipc } from '$lib/ipc';
  import { session } from '$lib/session.svelte';
  import type { CaseProfile } from '$lib/types/case';
  let title = $state('');
  let kind = $state($page.url.searchParams.get('kind') || 'document');
  let mode = $state('standard');
  let busy = $state(false);
  let error = $state('');
  const kinds = [
    ['document', 'Document project'], ['will', 'Will / estate document'], ['lease', 'Lease'], ['contractor', 'Agreement'], ['power_of_attorney', 'Power of attorney'],
    ['notice_of_appeal', 'Appeal'], ['reconsideration', 'Reconsideration'], ['enlargement', 'Extension of time'], ['stay', 'Stay'], ['amend_findings', 'Amend findings'], ['appellate_brief', 'Appellate brief'], ['appellate_motion', 'Appellate motion'], ['mandamus', 'Mandamus'], ['complaint', 'Complaint'], ['records_request', 'Records request'], ['confidential', 'Report Fraud / Qui Tam (EO 14395)'],
  ];
  async function create() {
    if (!title.trim()) { error = 'Give this matter a name.'; return; }
    if (mode === 'confidential' && !session.unlocked) { error = 'Unlock the Report Fraud / Qui Tam vault first, then return to create this matter.'; return; }
    busy = true; error = '';
    try {
      const created = await ipc<CaseProfile>('create_case', { title: title.trim(), documentKind: kind, mode });
      if (kind === 'document') {
        created.matter_tier = 'transactional';
        await ipc('save_case', { caseId: created.id, profile: created });
      }
      await goto('/case/' + created.id);
    } catch (e) { error = String(e); } finally { busy = false; }
  }
</script>
<main class="page-content" style="max-width: 850px">
  <header class="page-header"><div><a class="eyebrow" href="/">Workspace</a><h1 class="mt-2">New matter</h1><p>Create a place for your documents. You and your agent can fill in the details as you go.</p></div></header>
  <form class="panel p-6 space-y-6" onsubmit={(e) => { e.preventDefault(); create(); }}>
    <label class="block space-y-2"><span class="font-medium">Matter name</span><input class="field" bind:value={title} placeholder="A name you’ll recognize" required /></label>
    <label class="block space-y-2"><span class="font-medium">Type of work</span><select class="field" bind:value={kind} onchange={() => { if (kind === 'confidential') mode = 'confidential'; }}>{#each kinds as [id, label]}<option value={id}>{label}</option>{/each}</select><span class="text-xs text-muted-foreground block">This organizes the workspace. Available forms and rule packs determine what you can prepare.</span></label>
    <fieldset class="space-y-3"><legend class="font-medium mb-3">Purpose & storage</legend><label class="flex gap-3 items-start"><input type="radio" bind:group={mode} value="standard" class="mt-1" /><span><strong class="font-medium">Standard workspace</strong><span class="block description text-sm">Saved in your local documents folder. Optional sync with JustLegal.me.</span></span></label><label class="flex gap-3 items-start"><input type="radio" bind:group={mode} value="confidential" class="mt-1" /><span><strong class="font-medium">Report Fraud / Qui Tam (Confidential Vault)</strong><span class="block description text-sm">Protected local workflow for False Claims Act (EO 14395) & whistleblower evidence. Case material stays strictly off the network to preserve First-to-File priority.</span></span></label></fieldset>
    {#if mode === 'confidential' && !session.unlocked}<p class="notice">The protected vault needs to be unlocked first. <a class="text-primary underline" href="/confidential">Open Report Fraud / Qui Tam vault</a></p>{/if}
    {#if error}<p class="notice error" role="alert">{error}</p>{/if}
    <div class="toolbar"><button type="submit" class="button primary" disabled={busy || (mode === 'confidential' && !session.unlocked)}>{busy ? 'Creating…' : 'Create matter'}</button><a href="/" class="button quiet">Cancel</a></div>
  </form>
  <p class="text-sm text-muted-foreground">Looking for a specific document? <a href="/forms" class="text-primary underline">Browse forms and templates</a> without creating a matter first.</p>
</main>
