<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { Copy, Check, Terminal, ArrowRight, RefreshCw } from "lucide-svelte";
  import { ipc, runningInTauri } from "$lib/ipc";
  import { buildAgentBriefing, canIncludeMatter, cliCommands } from "$lib/agentBriefing";
  import type { WorkspaceCase } from "$lib/types/case";

  let cases = $state<WorkspaceCase[]>([]);
  let selectedId = $state("");
  let casesRoot = $state("");
  let cliPath = $state("");
  let loading = $state(true);
  let error = $state("");
  let copied = $state(false);
  let copiedItem = $state("");
  let showBriefing = $state(false);
  let showMcpJson = $state(false);
  let search = $state("");
  let doctor = $state<Record<string, unknown> | null>(null);
  let checking = $state(false);
  let copyTimer: ReturnType<typeof setTimeout>;
  const preview = !runningInTauri();
  const briefing = $derived(buildAgentBriefing({ cliPath, casesRoot, matter: cases.find(c => c.profile.id === selectedId), preview }));
  const groups = [...new Set(cliCommands.map(c => c.command.split(' ')[0]))];
  const mcpConfigJson = $derived(JSON.stringify({
    mcpServers: {
      "justlegal-os": {
        command: cliPath || "JustLegalOS-CLI.exe",
        args: ["mcp"]
      }
    }
  }, null, 2));

  const FRAUD_BOT_PROMPT = `IDENTITY & PURPOSE:
You are the JustLegal Fraud & Whistleblower Navigator, a specialized AI assistant sponsored by Just Liberty Inc. (501(c)(3)).
Your purpose is to help whistleblowers, relators, and citizen watchdogs organize, verify, and standardize evidence of federal government program fraud under Executive Order 14395 and the federal False Claims Act (31 U.S.C. §§ 3729–3733), covering both Street-Level Storefront Fraud and Systemic Institutional/Contractor Fraud.

CALLING CARD STANDARD (EO-14395):
Every evaluation must output the standardized ASCII Calling Card:
======================================================================
[JUSTLEGAL OS | JUSTLEGAL STANDARDIZED FRAUD DISCLOSURE v1.0 | EO-14395]
Generated via JustLegal.me / Just Liberty Inc. (501c3) Verification Engine
======================================================================

THE DUAL-TRACK & INSTITUTIONAL ROUTING ENGINE:
1. TRACK A (Relator Reward Track — Qui Tam Counsel First):
   - For $1M+ fraud and firsthand insider evidence (15%–30% relator share under 31 U.S.C. § 3730(d)).
   - Strictly enforced in camera court seal (31 U.S.C. § 3730(b)(2)). NEVER submit to public web portals or social media first!
2. TRACK B (Citizen Watchdog / Administrative Enforcement Track):
   - For sub-$1M or localized facility abuse. Formats pre-trimmed (<2,000 char) blocks for HHS-OIG, DOL-OIG, DOJ, and State MFCU portals.
3. GOVERNMENT TARGET & CONTRACTOR ENGINE:
   - Cites Vermont ANR v. Stevens (529 U.S. 765): State agencies cannot be FCA qui tam defendants.
   - Pivots to Private Prime Contractors (Deloitte, Maximus) under 31 U.S.C. § 3729(b)(2)(A)(ii).
   - Audits Sovereign Immunity bypasses (Ex parte Young, ADA Title II abrogation under United States v. Georgia, § 504 waivers, Monell).
   - Directs pre-filing open records requests (CORA, FOIA, NC DES, § 603.5(d)) with 501(c)(3) fee waivers.
   - Employs FRCP Rule 20(a)(1) Permissive Joinder for multi-plaintiff pro se actions splitting one filing fee.

Operate JustLegalOS-CLI (--json qui-tam, records, accountability-map, claim translate, sovereign-immunity, collective-action, receiver export) when local tool access is available. Never invent facts or citations.`;

  const CIVIL_BOT_PROMPT = `You are the JustLegal Civil Navigator sponsored by Just Liberty Inc. (501(c)(3)).
Your role is to assist self-represented (pro se) litigants in understanding court orders, finding official state/federal court forms from the 22,936-form catalog, calculating deterministic deadlines with holiday tolling, and preparing court-formatted filings via JustLegalOS-CLI without inventing citations or practicing law.`;

  onMount(() => {
    load();
    return () => clearTimeout(copyTimer);
  });
  async function load() {
    loading = true;
    error = "";
    try {
      const results = await Promise.allSettled([
        ipc<WorkspaceCase[]>("list_workspace_cases"),
        ipc<string>("get_cases_root"),
        ipc<string>("resolve_cli_path_cmd")
      ]);
      if (results[0].status === 'fulfilled') cases = results[0].value;
      else error = "Matters could not be loaded. The general agent briefing is still available.";
      if (results[1].status === 'fulfilled') casesRoot = results[1].value;
      const requested = $page.url.searchParams.get('case');
      if (!preview && requested && cases.some(c => c.profile.id === requested && canIncludeMatter(c))) selectedId = requested;
      if (!preview) {
        if (results[2].status === 'fulfilled' && results[2].value) {
          cliPath = results[2].value;
        } else {
          const { resourceDir, join } = await import('@tauri-apps/api/path');
          const dir = await resourceDir();
          cliPath = await join(dir, 'JustLegalOS-CLI.exe');
        }
      }
    } catch {
      error = "The CLI location could not be detected. Enter its full path below; your agent will verify it.";
    } finally { loading = false; }
  }
  async function copyBriefing() {
    error = "";
    try {
      await navigator.clipboard.writeText(briefing);
      copied = true;
      copiedItem = "briefing";
      clearTimeout(copyTimer);
      copyTimer = setTimeout(() => { copied = false; copiedItem = ""; }, 3500);
    } catch {
      showBriefing = true;
      error = "Clipboard access is unavailable. Select and copy the briefing below.";
    }
  }
  async function copyText(text: string, key: string) {
    error = "";
    try {
      await navigator.clipboard.writeText(text);
      copiedItem = key;
      clearTimeout(copyTimer);
      copyTimer = setTimeout(() => { copiedItem = ""; }, 3500);
    } catch {
      error = "Clipboard access is unavailable.";
    }
  }
  async function diagnose() {
    checking = true;
    error = "";
    try { doctor = await ipc<Record<string, unknown>>("doctor_diagnostic_cmd"); }
    catch (e) { error = String(e); }
    finally { checking = false; }
  }
</script>

<main class="page-content">
  <header class="page-header"><div><h1>AI</h1><p>Give your agent the tools to work with JustLegal OS.</p></div><span class="badge"><Terminal size={13} /> CLI · MCP · GrokBot</span></header>

  <section class="panel briefing-card">
    <div class="briefing-intro"><span class="eyebrow">Your agent, ready to work</span><h2>Start a conversation.<br />Bring the whole toolkit.</h2><p>Copy the briefing into a new chat with Gemini, Codex, Claude, Grok, or another agent with local tool access. Work together to organize files, fill in details, prepare documents, and check the results here.</p></div>
    <div class="briefing-action">
      <button class="button primary copy-button" onclick={copyBriefing} disabled={loading}>{#if copied}<Check size={18} /> Briefing copied{:else}<Copy size={18} /> Copy agent briefing{/if}</button>
      <p>Operating Charter & First-Run Bootstrap<br />Automatic Environment & Tool Detection</p>
      <button class="text-sm text-primary" onclick={() => showBriefing = !showBriefing}>{showBriefing ? 'Hide briefing' : 'Read the briefing'}</button>
      <span class="sr-only" role="status">{copied ? 'Agent briefing copied to clipboard.' : ''}</span>
    </div>
  </section>

  {#if error}<p role="alert" class="notice error">{error}</p>{/if}
  <div class="handoff-steps" aria-label="How it works">
    <div><span>1</span><p><strong>Copy & paste or connect MCP</strong>Give your agent the briefing or connect the local MCP server.</p></div>
    <div><span>2</span><p><strong>Work together</strong>Explain your goal and point to your files.</p></div>
    <div><span>3</span><p><strong>Review here</strong>See the saved documents and changes in your workspace.</p></div>
  </div>

  <section class="panel p-5 space-y-4">
    <div class="flex flex-wrap items-center justify-between gap-2">
      <div>
        <span class="eyebrow">GrokBot & MCP Connections</span>
        <h2 class="font-semibold text-base mt-1">Connect JustLegal Assistant or MCP Clients</h2>
      </div>
      {#if copiedItem && copiedItem !== "briefing"}
        <span class="badge">Copied {copiedItem}!</span>
      {/if}
    </div>
    <div class="grid gap-4 md:grid-cols-3">
      <div class="panel p-4 space-y-3">
        <span class="badge">GrokBot · EO 14395 & Qui Tam</span>
        <h3 class="font-semibold text-sm">Fraud & Whistleblower Navigator</h3>
        <p class="description text-xs">Verifies providers against NPI/USAspending, scores Rule 9(b) readiness, and routes Track A (Qui Tam Counsel) vs. Track B (Agency Portals).</p>
        <div class="toolbar">
          <button class="button" onclick={() => copyText(FRAUD_BOT_PROMPT, "Fraud Navigator prompt")}><Copy size={14} /> {copiedItem === "Fraud Navigator prompt" ? "Copied" : "Copy Bot Prompt"}</button>
          <a class="button quiet" href="https://grok.com" target="_blank" rel="noreferrer">Open Grok <ArrowRight size={13} /></a>
        </div>
      </div>

      <div class="panel p-4 space-y-3">
        <span class="badge">GrokBot · Civil & Appellate</span>
        <h3 class="font-semibold text-sm">JustLegal Civil Navigator</h3>
        <p class="description text-xs">Guides self-represented litigants through court orders, 22,936 official forms, deterministic deadline rules, and service certificates.</p>
        <div class="toolbar">
          <button class="button" onclick={() => copyText(CIVIL_BOT_PROMPT, "Civil Navigator prompt")}><Copy size={14} /> {copiedItem === "Civil Navigator prompt" ? "Copied" : "Copy Bot Prompt"}</button>
          <a class="button quiet" href="https://justlegal.me" target="_blank" rel="noreferrer">JustLegal.me <ArrowRight size={13} /></a>
        </div>
      </div>

      <div class="panel p-4 space-y-3">
        <span class="badge">Stdio JSON-RPC · Local Server</span>
        <h3 class="font-semibold text-sm">MCP Server Config</h3>
        <p class="description text-xs">Connect Claude Desktop, Cursor, Windsurf, or Antigravity directly to <code>JustLegalOS-CLI.exe mcp</code>.</p>
        <div class="toolbar">
          <button class="button" onclick={() => copyText(mcpConfigJson, "MCP JSON config")}><Copy size={14} /> {copiedItem === "MCP JSON config" ? "Copied" : "Copy MCP JSON"}</button>
          <button class="button quiet" onclick={() => (showMcpJson = !showMcpJson)}>{showMcpJson ? "Hide JSON" : "View JSON"}</button>
        </div>
      </div>
    </div>
    {#if showMcpJson}
      <pre>{mcpConfigJson}</pre>
    {/if}
  </section>

  <section class="panel p-5 space-y-4">
    <div class="grid gap-5 md:grid-cols-2">
      <label class="space-y-2"><span class="text-sm font-medium">CLI executable</span><input class="field" bind:value={cliPath} placeholder={preview ? 'Enter the full installed CLI path' : 'Locating the CLI…'} /><span class="block text-xs text-muted-foreground">Your agent verifies this location before running commands.</span></label>
      <label class="space-y-2"><span class="text-sm font-medium">Starting workspace</span><select class="field" bind:value={selectedId} disabled={preview}><option value="">General briefing — choose a matter with your agent</option>{#each cases.filter(canIncludeMatter) as c}<option value={c.profile.id}>{c.profile.title}</option>{/each}</select><span class="block text-xs text-muted-foreground">Only the selected standard matter’s folder path is included. Documents and Report Fraud / Qui Tam confidential matters are excluded.</span></label>
    </div>
    {#if preview}<p class="text-xs text-muted-foreground">Browser preview: no installed CLI or live connection has been verified.</p>{/if}
  </section>

  {#if showBriefing}
    <section class="panel p-5 space-y-3"><div class="flex items-center justify-between"><h2 class="font-semibold">Agent briefing</h2><span class="text-xs text-muted-foreground">{briefing.length.toLocaleString()} characters</span></div><textarea class="field briefing-text" aria-label="Agent briefing" readonly value={briefing} onfocus={(event) => event.currentTarget.select()}></textarea></section>
  {/if}

  <details class="panel p-5">
    <summary class="font-medium">Explore the CLI toolkit <span class="text-muted-foreground text-xs ml-2">{cliCommands.length} commands</span></summary>
    <p class="description text-xs mb-4">Captured from CLI help. Your agent checks the installed version before using a command.</p>
    <input class="field mb-4" aria-label="Search CLI commands" placeholder="Find a command, such as record, intake, pdf…" bind:value={search} />
    <div class="command-groups">
      {#each groups as group}
        {@const commands = cliCommands.filter(c => c.command.split(' ')[0] === group && (c.command + ' ' + c.help).toLowerCase().includes(search.toLowerCase()))}
        {#if commands.length}<details><summary>{group} <span class="text-xs text-muted-foreground">{commands.length}</span></summary>{#each commands as command}<details class="command"><summary>{command.command}</summary><pre>{command.help}</pre></details>{/each}</details>{/if}
      {/each}
      {#if !cliCommands.some(c => (c.command + ' ' + c.help).toLowerCase().includes(search.toLowerCase()))}<p class="description">No commands match this search.</p>{/if}
    </div>
  </details>
  <details class="panel p-5">
    <summary class="font-medium">Connection details & diagnostics</summary>
    <div class="space-y-4 mt-4 text-sm text-muted-foreground">
      <p>Terminal access is enough to use the CLI. For MCP clients, configure the executable above as the server command with the argument <code>mcp</code>. The agent discovers the server’s tools through MCP.</p>
      <p>Copying the briefing does not establish a connection. Your external agent confirms access by running the installed tools. This page does not host an AI conversation.</p>
      <button class="button" onclick={diagnose} disabled={checking}><RefreshCw size={14} />{checking ? 'Checking…' : 'Check local engine'}</button>
      {#if doctor}<pre class="diagnostics">{JSON.stringify(doctor, null, 2)}</pre><p class="text-xs">{preview ? 'Sample diagnostic data in browser preview.' : 'Local engine diagnostics; this does not verify an external agent connection.'}</p>{/if}
      <p class="text-xs">Report Fraud / Qui Tam confidential material requires a fully local agent environment. The generic briefing can be shared without including any confidential matter.</p>
    </div>
  </details>
</main>

<style>
  .briefing-card { padding: 32px; display: grid; grid-template-columns: minmax(0, 1.4fr) minmax(220px, 1fr); gap: 30px; background: linear-gradient(115deg, #fff, #f2f6fc); }
  .briefing-intro h2 { font-size: 30px; line-height: 1.22; letter-spacing: -.8px; font-weight: 600; margin: 12px 0 16px; }
  .briefing-intro p { color: hsl(var(--muted-foreground)); max-width: 530px; font-size: 14px; line-height: 1.7; }
  .briefing-action { display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 17px; text-align: center; }
  .copy-button { padding: 13px 20px; font-size: 14px; }
  .briefing-action p { font-size: 12px; color: hsl(var(--muted-foreground)); line-height: 1.8; }
  .handoff-steps { display: grid; grid-template-columns: repeat(3, 1fr); gap: 24px; padding: 0 5px; }
  .handoff-steps > div { display: flex; gap: 12px; }
  .handoff-steps span { display: grid; place-items: center; width: 26px; height: 26px; border-radius: 50%; background: #e5eaf2; font-size: 12px; flex-shrink: 0; }
  .handoff-steps p { font-size: 12px; color: hsl(var(--muted-foreground)); line-height: 1.7; }
  .handoff-steps strong { display: block; color: hsl(var(--foreground)); font-size: 13px; margin-bottom: 3px; }
  .briefing-text { height: 380px; font: 12px/1.7 ui-monospace, monospace; }
  .command-groups { display: grid; gap: 12px; }
  .command-groups > details { border-bottom: 1px solid hsl(var(--border)); padding-bottom: 10px; }
  .command { margin: 12px 0 0 16px; font-size: 13px; }
  pre { white-space: pre-wrap; overflow-wrap: anywhere; padding: 15px; background: hsl(var(--background)); font-size: 12px; line-height: 1.7; margin-top: 12px; }
  @media (max-width: 950px) { .briefing-card { grid-template-columns: 1fr; padding: 24px; } .briefing-action { align-items: flex-start; text-align: left; gap: 10px; } .briefing-intro h2 { font-size: 26px; } }
  @media (max-width: 700px) { .handoff-steps { grid-template-columns: 1fr; gap: 12px; } }
</style>
