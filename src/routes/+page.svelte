<script lang="ts">
  import { observeWorkspace } from "$lib/workspaceRefresh";
  import { ArrowUpRight, FolderOpen, Plus, Search, RefreshCw } from "lucide-svelte";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import CloudSyncModal from "$lib/components/CloudSyncModal.svelte";
  import PdfToolsModal from "$lib/components/PdfToolsModal.svelte";
  import SignatureStudioModal from "$lib/components/SignatureStudioModal.svelte";
  import PromptDialog from "$lib/components/PromptDialog.svelte";
  import { ipc } from "$lib/ipc";
  import { session } from "$lib/session.svelte";
  import type { WorkspaceCase, CloudSession, FolderImportReport, BootstrapResult, PortfolioStatusReport, PortfolioCaseItem } from "$lib/types/case";

  let cases = $state<WorkspaceCase[]>([]);
  let casesRoot = $state("");
  let portfolio = $state<PortfolioStatusReport | null>(null);
  let loadError = $state("");
  let actionError = $state("");
  let status = $state("");
  let showCloudModal = $state(false);
  let showPdfToolsModal = $state(false);
  let showSignatureModal = $state(false);
  let cloudSession = $state<CloudSession | null>(null);
  let promptKind = $state<"folder" | "osb" | "import" | "bootstrap" | null>(null);
  let filter = $state("");
  let loading = $state(true);
  let toolsOpen = $state(false);

  onMount(() => {
    load();
    checkCloud();
    return observeWorkspace(load);
  });

  async function checkCloud() {
    try {
      cloudSession = await ipc<CloudSession | null>("cloud_status_cmd");
    } catch {
      cloudSession = null;
    }
  }

  async function load() {
    try {
      casesRoot = await ipc<string>("get_cases_root");
      cases = await ipc<WorkspaceCase[]>("list_workspace_cases");
      portfolio = await ipc<PortfolioStatusReport>("portfolio_status_cmd").catch(() => null);
      loadError = "";
    } catch (e) {
      loadError = String(e);
    } finally { loading = false; }
  }



  async function handlePrompt(value: string) {
    const kind = promptKind;
    promptKind = null;
    actionError = "";
    if (!value.trim()) { actionError = "Enter the folder or file path."; return; }
    try {
      if (kind === "folder") {
        const profile = await ipc<{ id: string }>("open_case_folder", { path: value });
        goto(`/case/${profile.id}`);
      } else if (kind === "osb") {
        if (!session.unlocked) {
          actionError = "Unlock Confidential mode first. Import reads a vault bundle (.osb).";
          return;
        }
        const id = await ipc<string>("import_hunt_cmd", { osbPath: value });
        status = "Imported the confidential bundle.";
        await load();
        goto(`/case/${id}`);
      } else if (kind === "import") {
        const report = await ipc<FolderImportReport>("import_folder_cmd", {
          from: value,
          title: "Imported matter",
        });
        status = report.notice;
        await load();
        if (report.case_id) goto(`/case/${report.case_id}`);
      } else if (kind === "bootstrap") {
        const res = await ipc<BootstrapResult>("case_bootstrap_cmd", {
          sourceFolder: value,
          destinationParent: null,
          caseTitle: null,
        });
        status = res.summary;
        await load();
        goto(`/case/${res.case_id}`);
      }
    } catch (e) {
      actionError = String(e);
    }
  }

  async function trySample() {
    actionError = "";
    try {
      const profile = await ipc<{ id: string }>("seed_sample_cmd");
      status = "Opened sample case (Lucille Example). Nothing was filed.";
      await load();
      goto(`/case/${profile.id}`);
    } catch (e) {
      actionError = String(e);
    }
  }

  function getMatterUrgency(caseId: string): PortfolioCaseItem | undefined {
    return portfolio?.matters?.find((m) => m.case_id === caseId);
  }
</script>

<main class="page-content">
  <header class="page-header">
    <div><h1>Workspace</h1><p>Your matters, documents, and work in progress.</p></div>
    <div class="toolbar">
      <button class="button" onclick={() => (showCloudModal = true)}>
        {cloudSession ? `Cloud: ${cloudSession.email}` : "JustLegal.me Sync"}
      </button>
      <button class="button" onclick={() => (promptKind = "folder")}><FolderOpen size={16} /> Open</button>
      <a class="button primary" href="/case/new"><Plus size={16} /> New matter</a>
    </div>
  </header>

  {#if actionError || loadError}<p role="alert" class="notice error">{actionError || loadError}</p>{/if}
  {#if status}<p role="status" class="notice">{status}</p>{/if}

  <section class="agent-start panel">
    <div><span class="eyebrow">Start with your agent</span><h2>Your conversation. Your local workspace.</h2><p>Open the AI tab, copy the briefing or connect GrokBot / MCP, and work together here.</p></div>
    <a href="/ai" class="button primary">Set up your agent <ArrowUpRight size={16} /></a>
  </section>

  <section class="panel">
    <div class="panel-heading">
      <h2>My matters <span class="badge ml-2">{cases.length}</span></h2>
      <div class="toolbar">
        <label class="matter-search"><Search size={15} /><input aria-label="Search matters" placeholder="Find a matter" bind:value={filter} /></label>
        <button class="button quiet" onclick={load} aria-label="Refresh matters"><RefreshCw size={15} /></button>
      </div>
    </div>
    {#if loading}
      <div class="empty-state">Loading your workspace…</div>
    {:else if loadError}
      <div class="empty-state"><h3>Workspace could not be loaded</h3><button class="button" onclick={load}>Try again</button></div>
    {:else if cases.length === 0}
      <div class="empty-state"><h2>A place for everything you’re working on</h2><p>Your agent can create a matter, organize an existing folder, or help prepare a document.</p><div class="toolbar justify-center mt-5"><button class="button" onclick={() => (promptKind = "import")}>Import a folder</button><a class="button" href="/forms">Browse forms</a></div></div>
    {:else}
      <div class="matter-list">
        {#each cases.filter(c => (c.profile.title + ' ' + c.profile.docket_number).toLowerCase().includes(filter.toLowerCase())) as item}
          {@const urgency = getMatterUrgency(item.profile.id)}
          <a href={'/case/' + item.profile.id} class="matter-row">
            <div class="min-w-0"><h3>{item.profile.title || "Untitled matter"}</h3><p>{[item.profile.court.name, item.profile.docket_number].filter(Boolean).join(' · ') || item.profile.document_kind.replaceAll('_', ' ')}</p></div>
            <div class="matter-meta">
              {#if item.sealed || item.profile.mode === 'confidential'}<span class="badge">Qui Tam · Confidential</span>{/if}
              {#if urgency?.urgent_deadline}<span class="text-xs text-muted-foreground">{urgency.urgent_deadline}</span>{/if}
              <ArrowUpRight size={16} />
            </div>
          </a>
        {:else}<div class="empty-state">No matters match “{filter}”.</div>{/each}
      </div>
    {/if}
  </section>

  <div class="workspace-shortcuts">
    <a href="/forms" class="panel shortcut"><span class="eyebrow">Documents & everyday matters</span><h2>Find a form or template <span>→</span></h2><p>Agreements, estate documents, family matters, and court forms.</p></a>
    <button class="panel shortcut" onclick={() => (promptKind = "bootstrap")}><span class="eyebrow">Existing files</span><h2>Organize a case folder <span>→</span></h2><p>Case Rescue copies and organizes your papers while preserving originals.</p></button>
    <a href="/confidential" class="panel shortcut"><span class="eyebrow">EO 14395 · Whistleblower & False Claims</span><h2>Report Fraud / Qui Tam <span>→</span></h2><p>Verify providers, grade Rule 9(b) evidence, and prepare sealed disclosures in an airgapped local vault.</p></a>
  </div>

  <details class="workspace-utilities" bind:open={toolsOpen}>
    <summary>Workspace tools</summary>
    <div class="toolbar mt-3">
      <a class="button" href="/guide">Forms guide</a>
      <button class="button" onclick={() => (showPdfToolsModal = true)}>PDF tools</button>
      <button class="button" onclick={() => (showSignatureModal = true)}>Signature</button>
      <button class="button" onclick={() => (promptKind = "import")}>Import documents</button>
      <button class="button" onclick={() => (promptKind = "osb")}>Import vault bundle</button>
      <button class="button" onclick={() => (showCloudModal = true)}>JustLegal.me Sync</button>
      <button class="button" onclick={trySample}>Open sample matter</button>
    </div>
    <p class="text-xs text-muted-foreground mt-3 break-all">Workspace location: {casesRoot || "Unavailable"}</p>
  </details>

  <CloudSyncModal bind:open={showCloudModal} onclose={() => (showCloudModal = false)} onsynced={() => { load(); checkCloud(); }} />
  <PdfToolsModal bind:open={showPdfToolsModal} onclose={() => (showPdfToolsModal = false)} />
  <SignatureStudioModal bind:open={showSignatureModal} onclose={() => (showSignatureModal = false)} />
  <PromptDialog open={promptKind !== null}
    title={promptKind === "osb" ? "Import vault bundle" : promptKind === "bootstrap" ? "Case Rescue" : promptKind === "import" ? "Import documents" : "Open a matter"}
    body={promptKind === "bootstrap" || promptKind === "import" ? "Enter the full folder path. Files are copied into a workspace; originals stay in place. Review extracted details before using them." : "Enter the full local folder or file path."}
    placeholder="Full local path" onsubmit={handlePrompt} oncancel={() => (promptKind = null)} />
</main>

<style>
  .agent-start { padding: 23px 25px; display: flex; align-items: center; justify-content: space-between; gap: 24px; background: #f7faff; border-color: #d3deee; }
  .agent-start h2 { font-size: 20px; font-weight: 600; letter-spacing: -.4px; margin: 7px 0; }
  .agent-start p { font-size: 13px; color: hsl(var(--muted-foreground)); line-height: 1.6; }
  .matter-search { display: flex; align-items: center; gap: 8px; color: hsl(var(--muted-foreground)); }
  .matter-search input { background: transparent; width: 150px; font-size: 13px; padding: 6px; }
  .matter-row { padding: 19px 20px; display: flex; align-items: center; justify-content: space-between; gap: 14px; border-bottom: 1px solid hsl(var(--border)); }
  .matter-row:last-child { border-bottom: 0; }
  .matter-row:hover { background: #f8faff; }
  .matter-row h3 { font-weight: 600; font-size: 14px; }
  .matter-row p { margin-top: 5px; font-size: 12px; color: hsl(var(--muted-foreground)); }
  .matter-meta { display: flex; align-items: center; gap: 16px; flex-shrink: 0; }
  .workspace-shortcuts { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 14px; }
  .shortcut { padding: 19px; text-align: left; }
  .shortcut:hover { border-color: #9eb6d8; }
  .shortcut h2 { font-size: 14px; font-weight: 600; margin: 10px 0 7px; }
  .shortcut h2 span { float: right; color: hsl(var(--muted-foreground)); }
  .shortcut p { font-size: 12px; color: hsl(var(--muted-foreground)); line-height: 1.65; }
  .workspace-utilities { font-size: 12px; color: hsl(var(--muted-foreground)); }
  @media (max-width: 1050px) { .workspace-shortcuts { grid-template-columns: 1fr; } .agent-start { align-items: flex-start; flex-direction: column; gap: 16px; } }
  @media (max-width: 640px) { .matter-search input { width: 95px; } .matter-meta { gap: 6px; } }
</style>
