<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { Search } from 'lucide-svelte';
  import { ipc, runningInTauri } from '$lib/ipc';
  import type { FormSearchResult, FetchFormResult, AttachFormResult, CaseRecommendations, PackList } from '$lib/types/packs';
  import type { WorkspaceCase } from '$lib/types/case';
  let forms = $state<FormSearchResult[]>([]);
  let cases = $state<WorkspaceCase[]>([]);
  let selectedCaseId = $state('');
  let query = $state('');
  let jurisdiction = $state('ALL');
  let category = $state('ALL');
  let source = $state('ALL');
  let loading = $state(true);
  let error = $state('');
  let status = $state('');
  let pending = $state('');
  let packs = $state<PackList | null>(null);
  let recommendations = $state<CaseRecommendations | null>(null);
  let recommended = $state(false);
  let recommendationError = $state('');
  let recommending = $state(false);
  let states = $state<string[]>([]);
  let request = 0;
  let recommendationRequest = 0;
  let timer: ReturnType<typeof setTimeout>;
  const categories = [{ id: 'ALL', label: 'All documents' }, { id: 'housing', label: 'Housing' }, { id: 'family', label: 'Family' }, { id: 'wills', label: 'Wills & estates' }, { id: 'contracts', label: 'Agreements' }, { id: 'court', label: 'Court forms' }];
  const visibleForms = $derived((recommended ? (recommendations?.recommendations.map(r => r.form) || []) : forms).filter(f => source === 'ALL' || f.source_type === source));
  const filtered = $derived(!!query || category !== 'ALL' || jurisdiction !== 'ALL' || source !== 'ALL');
  const confidential = $derived(cases.some(c => c.profile.id === selectedCaseId && (c.sealed || c.profile.sealed_at || c.profile.mode === 'confidential')));

  onMount(() => { initialize(); return () => clearTimeout(timer); });
  async function initialize() {
    category = $page.url.searchParams.get('category') || 'ALL';
    query = $page.url.searchParams.get('q') || '';
    const result = await Promise.allSettled([ipc<WorkspaceCase[]>('list_workspace_cases'), ipc<PackList>('list_form_packs_cmd'), ipc<FormSearchResult[]>('search_forms_cmd', { query: null, state: null, task: null })]);
    if (result[0].status === 'fulfilled') cases = result[0].value;
    if (result[1].status === 'fulfilled') packs = result[1].value;
    if (result[2].status === 'fulfilled') states = [...new Set(result[2].value.map(f => f.state))].sort();
    await search();
  }
  function scheduleSearch() { clearTimeout(timer); timer = setTimeout(search, 250); }
  async function search() {
    const id = ++request;
    loading = true; error = '';
    try {
      const result = await ipc<FormSearchResult[]>('search_forms_cmd', { query: query || null, state: jurisdiction === 'ALL' ? null : jurisdiction, task: category === 'ALL' ? null : category });
      if (id === request) forms = result;
    } catch (e) { if (id === request) { error = String(e); forms = []; } }
    finally { if (id === request) loading = false; }
  }
  async function selectMatter() {
    const id = ++recommendationRequest;
    recommendations = null; recommendationError = ''; recommending = false;
    if (!selectedCaseId) { recommended = false; return; }
    recommending = true;
    try { const result = await ipc<CaseRecommendations>('recommend_forms_cmd', { caseId: selectedCaseId }); if (id === recommendationRequest) recommendations = result; }
    catch (e) { if (id === recommendationRequest) recommendationError = String(e); }
    finally { if (id === recommendationRequest) recommending = false; }
  }
  async function fetchForm(item: FormSearchResult) {
    pending = item.id; error = ''; status = '';
    try {
      const result = await ipc<FetchFormResult>('fetch_form_cmd', { formId: item.id, allowNetwork: !confidential });
      if (!result.ok) throw new Error(result.message || 'Download failed.');
      status = result.sha256_verified ? 'Downloaded and verified: ' + item.title : result.message || 'Downloaded; verification was not confirmed.';
      await search(); if (selectedCaseId) await selectMatter();
    } catch (e) { error = String(e); } finally { pending = ''; }
  }
  async function attach(item: FormSearchResult, asTemplate: boolean) {
    if (!selectedCaseId) return;
    pending = item.id; error = ''; status = '';
    try {
      const result = await ipc<AttachFormResult>('attach_form_cmd', { caseId: selectedCaseId, formId: item.id, asTemplate });
      if (!result.ok) throw new Error(result.notice || 'Could not attach document.');
      status = result.notice + ' ' + result.attached_path;
    } catch (e) { error = String(e); } finally { pending = ''; }
  }
  function clearFilters() { query = ''; jurisdiction = 'ALL'; category = 'ALL'; source = 'ALL'; recommended = false; search(); }
</script>

<main class="page-content">
  <header class="page-header"><div><h1>Forms & templates</h1><p>Find a document for what you’re working on.</p></div><a class="button" href="/packs">Manage packs</a></header>
  {#if !runningInTauri()}<p class="notice">Browser preview. Open the desktop app to search your installed forms catalog and attach documents.</p>{/if}
  <section class="panel p-5 space-y-4">
    <div class="search-bar"><label class="search-input"><Search size={18} /><input aria-label="Search forms" placeholder="Search forms, agreements, or document numbers" bind:value={query} oninput={scheduleSearch} /></label><select class="field" aria-label="Jurisdiction" bind:value={jurisdiction} onchange={search}><option value="ALL">All jurisdictions</option>{#each states as state}<option value={state}>{state}</option>{/each}</select><select class="field" aria-label="Document source" bind:value={source}><option value="ALL">All sources</option><option value="official_pdf">Official forms</option><option value="editable_template">Templates</option><option value="guide">Guides</option></select></div>
    <div class="toolbar">{#each categories as item}<button class="category" class:chosen={category === item.id} aria-pressed={category === item.id} onclick={() => { category = item.id; recommended = false; search(); }}>{item.label}</button>{/each}</div>
  </section>
  <div class="library-context"><span class="text-xs text-muted-foreground">Add documents to</span><select aria-label="Destination matter" class="field" bind:value={selectedCaseId} onchange={selectMatter}><option value="">Browse without a matter</option>{#each cases as item}<option value={item.profile.id}>{item.profile.title}</option>{/each}</select>{#if selectedCaseId}<button class="button" aria-pressed={recommended} onclick={() => recommended = !recommended}>{recommended ? 'Show catalog' : 'Suggestions for matter'}</button>{/if}</div>
  {#if error}<p class="notice error" role="alert">{error} <button class="underline ml-2" onclick={search}>Retry catalog</button></p>{/if}
  {#if status}<p class="notice break-words" role="status">{status}</p>{/if}
  {#if recommended && recommendationError}<p class="notice error" role="alert">Suggestions could not be loaded: {recommendationError}</p>{/if}
  {#if confidential}<p class="text-xs text-muted-foreground">Confidential matter selected. Network downloads are disabled; use available local forms.</p>{/if}
  <section aria-label="Form results" aria-busy={loading || recommending}>
    <div class="flex justify-between items-center mb-4"><h2 class="font-semibold">{recommended ? 'Suggestions for your matter' : 'Documents'}</h2><span class="text-xs text-muted-foreground">{visibleForms.length} results</span></div>
    {#if loading || (recommended && recommending)}<div class="panel empty-state">Loading documents…</div>
    {:else if error || (recommended && recommendationError)}<div class="panel empty-state"><h3>Documents could not be loaded</h3><p>Resolve the error above or retry.</p></div>
    {:else if visibleForms.length === 0}
      <div class="panel empty-state"><h3>{recommended ? 'No suggestions for this matter yet' : filtered ? 'No matching documents' : 'No documents in the current catalog'}</h3><p>{recommended ? 'Browse the catalog to find an available form.' : filtered ? 'Try another search or clear your filters.' : packs?.installed.length === 0 ? 'No form packs are installed. Open Manage packs to see available sources and local installation options.' : 'Open Manage packs to inspect installed resources and available catalog sources.'}</p><div class="toolbar justify-center mt-4"><button class="button" onclick={clearFilters}>{filtered || recommended ? 'Show all documents' : 'Refresh catalog'}</button><a class="button" href="/packs">Manage packs</a></div></div>
    {:else}
      <div class="form-results">{#each visibleForms as item}
        <article class="panel form-result"><div class="toolbar"><span class="badge">{item.source_type === 'official_pdf' ? 'Official form' : item.source_type === 'guide' ? 'Guide' : 'Template'}</span><span class="text-xs text-muted-foreground">{item.is_offline ? 'Available offline' : 'Download required'}</span></div><h3>{item.title}</h3><p class="text-xs text-muted-foreground">{[item.state, item.court, item.form_number].filter(Boolean).join(' · ')}</p>{#if item.teach.what_it_is}<p class="form-description">{item.teach.what_it_is}</p>{/if}
          {#if recommended}<p class="text-xs text-muted-foreground">{recommendations?.recommendations.find(r => r.form.id === item.id)?.relevance_reason}</p>{/if}
          {#if item.citation && item.citation_status !== 'pack_source'}<p class="text-xs text-amber-700">Citation needs verification.</p>{/if}
          <div class="form-actions"><span class="text-xs text-muted-foreground">{item.size_label}</span><div class="toolbar">{#if !item.is_offline}<button class="button" disabled={!!pending || confidential} onclick={() => fetchForm(item)}>{pending === item.id ? 'Downloading…' : 'Download'}</button>{:else if selectedCaseId}<button class="button" disabled={!!pending} onclick={() => attach(item, true)}>Use as draft</button><button class="button" disabled={!!pending} onclick={() => attach(item, false)}>Attach</button>{:else}<span class="text-xs text-muted-foreground">Select a matter to use</span>{/if}</div></div>
        </article>
      {/each}</div>
    {/if}
  </section>
</main>
<style>
  .search-bar { display: grid; grid-template-columns: minmax(180px, 1fr) 160px 150px; gap: 10px; }
  .search-input { border: 1px solid hsl(var(--input)); border-radius: 7px; padding: 0 11px; display: flex; align-items: center; gap: 9px; color: hsl(var(--muted-foreground)); }
  .search-input input { width: 100%; padding: 10px 0; background: transparent; color: hsl(var(--foreground)); }
  .category { padding: 6px 11px; border-radius: 6px; font-size: 12px; color: hsl(var(--muted-foreground)); }
  .category.chosen { background: #e5edf8; color: hsl(var(--primary)); font-weight: 600; }
  .library-context { display: flex; flex-wrap: wrap; align-items: center; gap: 10px; }
  .library-context select { max-width: 360px; font-size: 13px; }
  .form-results { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 14px; }
  .form-result { padding: 20px; display: flex; flex-direction: column; gap: 9px; }
  .form-result h3 { font-weight: 600; font-size: 15px; margin-top: 3px; }
  .form-description { font-size: 13px; line-height: 1.65; color: hsl(var(--muted-foreground)); }
  .form-actions { display: flex; flex-wrap: wrap; align-items: center; justify-content: space-between; gap: 8px; padding-top: 13px; border-top: 1px solid hsl(var(--border)); margin-top: auto; }
  @media (max-width: 1000px) { .search-bar { grid-template-columns: 1fr 1fr; } .search-input { grid-column: 1 / -1; } .form-results { grid-template-columns: 1fr; } }
</style>
