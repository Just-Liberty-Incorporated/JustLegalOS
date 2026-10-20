<script lang="ts">
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { isLitigationMatter } from "$lib/matterType";
  import { observeWorkspace } from "$lib/workspaceRefresh";
  import { RefreshCw, Save } from "lucide-svelte";
  import CasePartiesTab from "$lib/components/tabs/CasePartiesTab.svelte";
  import FieldHelp from "$lib/components/FieldHelp.svelte";
  import PreFlightAuditCard from "$lib/components/PreFlightAuditCard.svelte";
  import MotionDocket from "$lib/components/MotionDocket.svelte";
  import ServiceCertCard from "$lib/components/ServiceCertCard.svelte";
  import ProceduralGraphCard from "$lib/components/ProceduralGraphCard.svelte";
  import CaseOverviewTab from "$lib/components/tabs/CaseOverviewTab.svelte";
  import CaseDocumentsTab from "$lib/components/tabs/CaseDocumentsTab.svelte";
  import CaseEvidenceTab from "$lib/components/tabs/CaseEvidenceTab.svelte";
  import CaseTimelineTab from "$lib/components/tabs/CaseTimelineTab.svelte";
  import CaseCheckTab from "$lib/components/tabs/CaseCheckTab.svelte";
  import CaseFilesTab from "$lib/components/tabs/CaseFilesTab.svelte";
  import CaseConfidentialTab from "$lib/components/tabs/CaseConfidentialTab.svelte";
  import CaseRecordTab from "$lib/components/tabs/CaseRecordTab.svelte";
  import ReviewCourtOrderCard from "$lib/components/ReviewCourtOrderCard.svelte";
  import CaseModals from "$lib/components/CaseModals.svelte";
  import QuiTamModeBanner from "$lib/components/fraud/QuiTamModeBanner.svelte";
  import GhostFacilityWizardModal from "$lib/components/fraud/GhostFacilityWizardModal.svelte";
  import PostExportNextStepsModal from "$lib/components/PostExportNextStepsModal.svelte";
  import { decodeOrderForCaseHelper } from "$lib/utils/orderOperations";
  import { promptAndUploadEvidence } from "$lib/utils/evidenceOperations";
  import { ipc } from "$lib/ipc";
  import { session } from "$lib/session.svelte";
  import type {
    CaseArtifact,
    CaseProfile,
    ExportResult,
    Party,
    SyncRefusal,
    TimelineEvent,
    ValidationReport,
    GuidedDraft,
    ExhibitPacketReport,
    FormInspection,
    FormFillResult,
    CaseFilingItem,
    DecodedOrder,
    MailAuditReport,
  } from "$lib/types/case";

  let caseId = $derived($page.params.id || "");
  let profile = $state<CaseProfile | null>(null);
  let parties = $state<Party[]>([]);
  let addressText = $state("");
  let tab = $state<"overview" | "docket" | "record" | "service" | "parties" | "documents" | "evidence" | "timeline" | "files" | "check" | "confidential" | "tools">("overview");
  let confirmSeal = $state(false);
  let showCloudModal = $state(false);
  let showPdfToolsModal = $state(false);
  let showSignatureModal = $state(false);
  let showIntakeWizard = $state(false);
  let showAccountabilityModal = $state(false);
  let showAdaModal = $state(false);
  let showRecordRetrievalModal = $state(false);
  let showWorkflowModal = $state(false);
  let showConsultationModal = $state(false);
  let showGhostWizard = $state(false);
  let showPostExportModal = $state(false);
  let postExportFilePath = $state("");
  let postExportTitle = $state("");
  let preselectedIssueId = $state<string | null>(null);
  let activeFilingTitle = $state("");
  let exhibitPacketReport = $state<ExhibitPacketReport | null>(null);
  let batesPrefix = $state("");
  let batesStart = $state(1);
  let assemblingPacket = $state(false);
  let selectedFormPath = $state("");
  let formInspection = $state<FormInspection | null>(null);
  let formFillResult = $state<FormFillResult | null>(null);
  let fillingForm = $state(false);
  let checkReport = $state<{
    ok?: boolean;
    never_files?: boolean;
    issues?: { field: string; message: string }[];
    fact?: { rows: { id: number; assertion: string; record_cite: string; exhibit?: string; verified: boolean }[] };
    authority?: { rows: { id: number; cite: string; status: string }[] };
    deadline?: { due: string; math: string; rule: string };
    signoff?: { items: { id: string; label: string; checked: boolean }[]; ok?: boolean };
  } | null>(null);
  let guided = $state<{ kind: string; sections: { id: string; heading: string; hint: string; user_text: string }[] } | null>(null);
  let exhibits = $state<{ letter: string; filename: string; description: string }[]>([]);
  let gapTitle = $state("");
  let gapHard = $state("");
  let deadlinePack = $state("nj-app-div");
  let deadlineRule = $state("reconsideration-appellate");
  let triggerDate = $state("");
  let mdDraft = $state("");
  let error = $state("");
  let status = $state("");
  let report = $state<ValidationReport | null>(null);
  let syncResult = $state<SyncRefusal | null>(null);
  let artifacts = $state<CaseArtifact[]>([]);
  let exportPaths = $state<ExportResult | null>(null);
  let busy = $state(false);
  let sealPassword = $state("");
  let eventTitle = $state("");
  let eventDate = $state("");
  let eventNotes = $state("");
  let targetLookup = $state("");
  let targetHits = $state<{ recipient_name?: string; award_amount?: number; description?: string }[]>([]);
  let hydrated = $state("");
  let activeOrderAlert = $state<DecodedOrder | null>(null);
  let activeMailAudit = $state<MailAuditReport | null>(null);
  let inspectingOrder = $state(false);

  let savedSnapshot = $state("");
  let remoteSnapshot = "";
  let localInput = $state(false);
  let externalChanges = $state(false);
  let refreshError = $state("");
  let revision = $state(0);
  let refreshing = false;
  const dirty = $derived(!!savedSnapshot && (localInput || JSON.stringify({ profile, parties, addressText, mdDraft }) !== savedSnapshot));
  const litigation = $derived(isLitigationMatter(profile));
  onMount(() => { load(); return observeWorkspace(refreshFromAgent); });

  async function refreshFromAgent() {
    if (!profile || busy || refreshing || showIntakeWizard || showWorkflowModal || showConsultationModal || showAccountabilityModal || showAdaModal || showRecordRetrievalModal || showGhostWizard || showPdfToolsModal || showSignatureModal || showCloudModal) return;
    refreshing = true;
    try {
      const [next, files, sections] = await Promise.all([
        ipc<CaseProfile>("get_case", { caseId }),
        ipc<CaseArtifact[]>("list_case_artifacts", { caseId }),
        ipc<GuidedDraft>("guided_sections_cmd", { caseId }).catch(() => null),
      ]);
      if (busy) return;
      const raw = JSON.stringify(next);
      const changed = raw !== remoteSnapshot || JSON.stringify(files) !== JSON.stringify(artifacts) || JSON.stringify(sections) !== JSON.stringify(guided);
      if (changed && dirty) { externalChanges = true; return; }
      if (changed) {
        remoteSnapshot = raw;
        profile = next; hydrated = ''; hydrate(next);
        artifacts = files; guided = sections;
        savedSnapshot = JSON.stringify({ profile, parties, addressText, mdDraft });
        revision += 1; externalChanges = false;
      }
      refreshError = '';
    } catch (e) { refreshError = 'Could not refresh saved changes: ' + String(e); }
    finally { refreshing = false; }
  }

  function reloadSaved() { localInput = false; savedSnapshot = ''; hydrated = ''; return load(); }

  async function load() {
    error = "";
    try {
      profile = await ipc<CaseProfile>("get_case", { caseId });
      remoteSnapshot = JSON.stringify(profile);
      hydrated = ""; hydrate(profile);
      artifacts = await ipc<CaseArtifact[]>("list_case_artifacts", { caseId }).catch(() => []);
      guided = await ipc<GuidedDraft>("guided_sections_cmd", { caseId }).catch(() => null);
      const idx = await ipc<{ items?: { letter: string; filename: string; description: string }[] }>("exhibit_index_cmd", { caseId }).catch(() => ({ items: [] }));
      exhibits = idx.items || [];
      localInput = false; externalChanges = false;
      savedSnapshot = JSON.stringify({ profile, parties, addressText, mdDraft });
      revision += 1;
    } catch (e) {
      error = String(e);
    }
  }

  function hydrate(next: CaseProfile) {
    if (hydrated === next.id + next.updated_at) return;
    const left = next.caption.appellants.length ? next.caption.appellants : next.caption.plaintiffs;
    const right = next.caption.appellees.length ? next.caption.appellees : next.caption.defendants;
    parties = [...left, ...right].map((p) => ({
      name: p.name,
      role: p.role,
      address: p.address || "",
      email: p.email || "",
      phone: p.phone || "",
      counsel: p.counsel || "",
    }));
    addressText = next.filer.address_lines.join("\n");
    mdDraft = next.court_markdown || "";
    if (!next.complaint) next.complaint = { title: "", user_text: "" };
    if (!next.motion) next.motion = { title: "", user_text: "" };
    if (!next.brief) next.brief = { title: "", user_text: "" };
    if (!next.appellate_motion) next.appellate_motion = { title: "", user_text: "" };
    if (!next.mandamus) next.mandamus = { title: "", user_text: "" };
    if (!next.timeline) next.timeline = [];
    hydrated = next.id + next.updated_at;
  }

  function apply() {
    if (!profile) return;
    const left = parties.filter((p) => ["Plaintiff", "Appellant", "Petitioner"].includes(p.role));
    const right = parties.filter((p) => ["Defendant", "Appellee", "Respondent"].includes(p.role));
    const other = parties.filter((p) => !left.includes(p) && !right.includes(p));
    profile.caption.plaintiffs = left.length ? left : other;
    profile.caption.appellants = left;
    profile.caption.defendants = right;
    profile.caption.appellees = right;
    profile.court_markdown = mdDraft;
    profile.filer.address_lines = addressText.split("\n").map((l) => l.trim()).filter(Boolean);
  }

  async function save() {
    if (!profile || busy) return false;
    const current = await ipc<CaseProfile>("get_case", { caseId }).catch(() => null);
    if (!current || JSON.stringify(current) !== remoteSnapshot || externalChanges) {
      externalChanges = true;
      error = 'Saved data changed or could not be checked. Reload the saved version before writing; copy any unsaved text you want to keep.';
      return false;
    }
    apply();
    busy = true;
    error = "";
    try {
      const markdownChanged = mdDraft !== (JSON.parse(remoteSnapshot).court_markdown || '');
      profile = await ipc<CaseProfile>("save_case", { caseId, profile });
      if (markdownChanged && mdDraft.trim()) {
        await ipc("import_markdown_cmd", { caseId, markdown: mdDraft });
        profile = await ipc<CaseProfile>("get_case", { caseId });
      }
      remoteSnapshot = JSON.stringify(profile);
      hydrated = ''; hydrate(profile);
      remoteSnapshot = JSON.stringify(profile);
      localInput = false; externalChanges = false;
      savedSnapshot = JSON.stringify({ profile, parties, addressText, mdDraft });
      status = "Changes saved locally.";
      return true;
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  async function validate() {
    if (!await save()) return;
    try {
      report = await ipc<ValidationReport>("validate_case_cmd", { caseId });
      status = report.ok ? "Ready to export the selected document." : "Fix the issues below before export.";
    } catch (e) {
      error = String(e);
    }
  }

  async function exportKind(kind: string) {
    if (!await save()) return;
    busy = true;
    error = "";
    try {
      exportPaths = await ipc<ExportResult>("export_pleading_cmd", { caseId, kind });
      status = `Wrote Word and PDF:\n${exportPaths.docx}\n${exportPaths.pdf}`;
      artifacts = await ipc<CaseArtifact[]>("list_case_artifacts", { caseId }).catch(() => artifacts);
      postExportFilePath = exportPaths.pdf || exportPaths.docx || "";
      postExportTitle = profile?.title || kind;
      showPostExportModal = true;
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  async function trySync(action: string) {
    try {
      syncResult = await ipc<SyncRefusal>("sync_case_cmd", { caseId, action });
    } catch (e) {
      error = String(e);
    }
  }

  async function seal() {
    if (!sealPassword) {
      error = "Re-enter the Confidential vault password to seal. A wrong password is refused so files are not encrypted under an unknown key.";
      return;
    }
    if (!session.unlocked) {
      error = "Unlock Confidential mode first.";
      return;
    }
    confirmSeal = true;
  }

  async function doSeal() {
    confirmSeal = false;
    busy = true;
    try {
      await ipc("convert_case_to_confidential", { caseId, password: sealPassword });
      sealPassword = "";
      status = "Case sealed. The vault copy is encrypted. This build has no in-app reader for sealed files.";
      await load();
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  function addTimeline() {
    if (!profile || !eventTitle || !eventDate) {
      error = "Timeline events need a title and a date.";
      return;
    }
    const item: TimelineEvent = {
      id: crypto.randomUUID(),
      title: eventTitle,
      date: eventDate,
      notes: eventNotes,
    };
    profile.timeline = [...(profile.timeline || []), item];
    eventTitle = "";
    eventDate = "";
    eventNotes = "";
  }

  function removeTimeline(id: string) {
    if (!profile) return;
    profile.timeline = (profile.timeline || []).filter((e) => e.id !== id);
  }

  function addEvidence() {
    promptAndUploadEvidence(caseId, profile, (res) => {
      if (res.status) status = res.status;
      if (res.error) error = res.error;
      if (res.artifacts) artifacts = res.artifacts;
    });
  }

  async function lookupTarget() {
    if (!targetLookup.trim()) return;
    try {
      targetHits = await ipc("verify_target_cmd", { name: targetLookup.trim() });
    } catch (e) {
      error = String(e);
    }
  }

  async function disclosure() {
    if (!profile) return;
    const password = prompt("If this case is sealed, re-enter the vault password. Leave empty only for an unsealed Confidential case.");
    try {
      const path = await ipc<string>("save_disclosure_cmd", {
        huntId: caseId,
        target: profile.title,
        count: artifacts.filter((a) => a.kind === "evidence").length,
        value: 0,
        password: password && password.length ? password : null,
      });
      status = `Disclosure statement saved:\n${path}`;
    } catch (e) {
      error = String(e);
    }
  }

  async function fillSection(id: string, text: string) {
    if (!await save()) return;
    try {
      guided = await ipc<GuidedDraft>("guided_fill_cmd", { caseId, id, text });
      profile = await ipc<CaseProfile>("get_case", { caseId });
      remoteSnapshot = JSON.stringify(profile);
      hydrated = ''; hydrate(profile);
      localInput = false;
      savedSnapshot = JSON.stringify({ profile, parties, addressText, mdDraft });
      status = "Section saved locally.";
    } catch (e) {
      error = String(e);
    }
  }

  async function linkFact(factId: number, letter: string) {
    try {
      await ipc("link_fact_cmd", { caseId, factId, letter });
      await runChecks();
    } catch (e) {
      error = String(e);
    }
  }

  async function toggleSignoff(id: string, checked: boolean) {
    try {
      await ipc("set_signoff_cmd", { caseId, id, checked });
      await runChecks();
    } catch (e) {
      error = String(e);
    }
  }

  async function exportPacket() {
    if (!await save()) return;
    try {
      const report = await ipc<{ items?: { label: string; docx: string }[] }>("export_packet_cmd", { caseId });
      status = `Packet written. ${(report.items || []).map((i) => i.label).join(", ")}. You still file and serve.`;
    } catch (e) {
      error = String(e);
    }
  }

  async function exportIcs() {
    try {
      const path = await ipc<string>("export_ics_cmd", {
        caseId,
        pack: deadlinePack,
        rule: deadlineRule,
        triggerDate,
      });
      status = `Calendar file: ${path}`;
    } catch (e) {
      error = String(e);
    }
  }

  async function addGap() {
    if (!gapTitle.trim()) return;
    try {
      await ipc("gap_add_cmd", {
        caseId,
        title: gapTitle,
        court: profile?.court.name || "",
        rule: deadlineRule,
        whatHappened: gapTitle,
        whatWasHard: gapHard,
      });
      const dest = await ipc<string>("gap_export_cmd", { caseId });
      status = `Gap note saved. ${dest}`;
      gapTitle = "";
      gapHard = "";
    } catch (e) {
      error = String(e);
    }
  }

  async function runChecks() {
    try {
      checkReport = await ipc("check_case_cmd", {
        caseId,
        pack: deadlinePack,
        rule: deadlineRule,
        triggerDate: triggerDate || null,
      });
      status = checkReport?.ok
        ? "Check gates passed. The app still did not file or serve."
        : "Check found issues. See the list. The app never invents cites.";
    } catch (e) {
      error = String(e);
    }
  }

  async function exportOsb() {
    const path = prompt("Optional target path for the .osb file. Leave empty to save in Downloads.");
    try {
      const out = await ipc<string>("export_hunt_cmd", {
        huntId: caseId,
        targetPath: path && path.length ? path : "DOWNLOADS",
      });
      status = `Exported confidential bundle:\n${out}`;
    } catch (e) {
      error = String(e);
    }
  }

  async function generateExhibitPacket() {
    assemblingPacket = true;
    error = "";
    try {
      exhibitPacketReport = await ipc<ExhibitPacketReport>("assemble_exhibit_packet_cmd", {
        caseId,
        prefix: batesPrefix.trim() || null,
        startNumber: batesStart || 1,
      });
      status = `Exhibit packet assembled!\nTable: ${exhibitPacketReport.table_of_exhibits_pdf}\nPacket: ${exhibitPacketReport.exhibit_packet_pdf}\nTotal pages: ${exhibitPacketReport.total_pages}`;
      artifacts = await ipc<CaseArtifact[]>("list_case_artifacts", { caseId }).catch(() => artifacts);
      postExportFilePath = exhibitPacketReport.exhibit_packet_pdf || "";
      postExportTitle = "Assembled Exhibit Packet";
      showPostExportModal = true;
    } catch (e) {
      error = String(e);
    } finally {
      assemblingPacket = false;
    }
  }

  async function inspectSelectedForm(formPath: string) {
    selectedFormPath = formPath;
    error = "";
    try {
      formInspection = await ipc<FormInspection>("inspect_court_form_cmd", {
        formPath,
      });
      status = `Inspected form: ${formInspection.detected_form_number || formInspection.form_name} (${formInspection.fields.length} fields)`;
    } catch (e) {
      error = String(e);
    }
  }

  async function fillSelectedForm(formPath: string) {
    fillingForm = true;
    error = "";
    try {
      formFillResult = await ipc<FormFillResult>("fill_court_form_cmd", {
        caseId,
        formPath,
      });
      status = `Filled form generated!\nPDF: ${formFillResult.output_pdf_path}\nJSON: ${formFillResult.output_json_path}\nMapped fields: ${formFillResult.mapped_fields}`;
      artifacts = await ipc<CaseArtifact[]>("list_case_artifacts", { caseId }).catch(() => artifacts);
      postExportFilePath = formFillResult.output_pdf_path || "";
      postExportTitle = "Filled Court Form";
      showPostExportModal = true;
    } catch (e) {
      error = String(e);
    } finally {
      fillingForm = false;
    }
  }

  async function decodeOrderForCase(pdfPath?: string) {
    inspectingOrder = true;
    error = "";
    try {
      const res = await decodeOrderForCaseHelper(caseId, profile, artifacts, pdfPath);
      if (res) {
        activeOrderAlert = res.order;
        activeMailAudit = res.mailAudit;
        status = `Decoded order: ${res.order.title}`;
        if (res.ingested) {
          status = `Ingested & preserved order in record: ${res.order.title}`;
          await load();
        }
      }
    } catch (e) {
      error = String(e);
    } finally {
      inspectingOrder = false;
    }
  }

  function handleOrderAction(action: string) {
    status = `Selected order action: ${action}`;
    if (action.toLowerCase().includes("reconsideration") || action.toLowerCase().includes("motion")) {
      tab = "docket";
    } else if (action.toLowerCase().includes("transcript") || action.toLowerCase().includes("record")) {
      tab = "record";
    }
  }
</script>

<main class="page-content case-page">
  {#if !profile}
    <p class="notice" role="status">{error || "Loading matter…"}</p>
  {:else}
    <header class="page-header">
      <div><a href="/" class="eyebrow">Workspace / Matter</a><h1 class="mt-2">{profile.title || "Untitled matter"}</h1><p>{[profile.court.name, profile.docket_number].filter(Boolean).join(' · ') || profile.document_kind.replaceAll('_', ' ')}</p></div>
      <div class="toolbar"><span class="text-xs text-muted-foreground">{dirty ? 'Unsaved changes' : 'Saved locally'}</span>{#if profile.mode === 'standard' && !profile.sealed_at}<button class="button" onclick={() => (showCloudModal = true)}>JustLegal.me Sync</button>{/if}<button class="button" onclick={refreshFromAgent} disabled={busy}><RefreshCw size={15} /> Refresh</button><button class="button primary" onclick={save} disabled={busy || !!profile.sealed_at || externalChanges}><Save size={15} /> Save</button></div>
    </header>
    {#if externalChanges}<div class="notice" role="status">Your agent or another window changed this matter. Unsaved edits have been kept on screen. Copy any text you want to preserve before loading the saved version. <button class="button mt-2" onclick={reloadSaved}>Reload saved version · discard local edits</button></div>{/if}
    {#if refreshError}<p class="notice error" role="status">{refreshError}</p>{/if}
    {#if profile.sealed_at}<p class="notice">Sealed matter. File access uses the unlocked vault.</p>{/if}
    {#if activeOrderAlert}
      <ReviewCourtOrderCard order={activeOrderAlert} mailAudit={activeMailAudit} onSelectAction={handleOrderAction} onDismiss={() => { activeOrderAlert = null; }} />
    {/if}

    <div class="matter-layout">
    <nav class="matter-navigation" aria-label="Matter sections">
      {#each [
        ['overview', 'Overview'], ['documents', 'Documents'], ['files', 'Files & exports'],
        ['record', litigation ? 'Court record' : 'Records'], ['evidence', 'Evidence'],
        ['timeline', 'Timeline'], ['parties', 'People'],
        ...(litigation ? [['docket', 'Motions & filings'], ['service', 'Service & orders']] : []),
        ['check', 'Checks'], ['tools', 'Tools & settings'],
        ...(profile.mode === 'confidential' ? [['confidential', 'Qui Tam & Fraud Triage']] : [])
      ] as [id, label]}
        <button class:chosen={tab === id} aria-current={tab === id ? 'page' : undefined} onclick={() => tab = id as typeof tab}>{label}</button>
      {/each}
    </nav>
    <div class="matter-body" oninput={() => localInput = true} onchange={() => localInput = true} role="group" aria-label="Matter content">
    {#key revision}
    {#if tab === "overview"}
      <section class="panel p-6 space-y-5">
        <div><span class="eyebrow">Current workspace</span><h2 class="text-xl font-semibold mt-2">Your work at a glance</h2><p class="description text-sm">Review the documents and details saved by you and your agent.</p></div>
        <div class="overview-stats"><button onclick={() => tab = 'files'}><strong>{artifacts.length}</strong><span>Files & exports</span></button><button onclick={() => tab = 'evidence'}><strong>{exhibits.length}</strong><span>Indexed exhibits</span></button><button onclick={() => tab = 'timeline'}><strong>{profile.timeline?.length || 0}</strong><span>Timeline entries</span></button></div>
        <div class="toolbar"><button class="button" onclick={() => tab = 'documents'}>Open documents</button><button class="button" onclick={() => showIntakeWizard = true}>Complete missing details</button>{#if litigation}<button class="button" onclick={() => decodeOrderForCase()}>Read a court order</button>{/if}{#if profile.mode === 'confidential'}<button class="button" onclick={() => showGhostWizard = true}>Ghost Facility & Rule 9(b) Wizard</button>{/if}</div>
      </section>
      {#if profile.receipts?.length}<section class="panel p-5 space-y-3"><h2 class="font-semibold">Recent activity</h2>{#each profile.receipts.slice(-5).reverse() as receipt}<div class="text-sm border-b pb-2"><p>{receipt.operation} <span class="badge">{receipt.status}</span></p><p class="text-xs text-muted-foreground mt-1">{receipt.timestamp} · {receipt.invoked_by}</p>{#if receipt.files_modified?.length}<p class="text-xs text-muted-foreground break-all">{receipt.files_modified.join(', ')}</p>{/if}</div>{/each}</section>{/if}
      <details class="panel p-5"><summary class="font-medium">Matter details & contact information</summary><div class="mt-4"><CaseOverviewTab bind:profile bind:addressText /></div></details>
    {:else if tab === "docket"}
      <section class="space-y-4">
        <MotionDocket caseId={caseId} onselectmotion={(m) => { activeFilingTitle = m.title; }} onexport={(paths) => {
          exportPaths = { docx: paths.docx, pdf: paths.pdf, notice: "Review every line before filing." };
          status = `Exported ${paths.pdf}`;
          postExportFilePath = paths.pdf || paths.docx;
          postExportTitle = activeFilingTitle || profile?.title || "Court Motion";
          showPostExportModal = true;
        }} />
      </section>
    {:else if tab === "record"}
      <CaseRecordTab {caseId} {profile} onExploreRoutes={(id) => { preselectedIssueId = id || null; showAccountabilityModal = true; }}
        onRequestFormReady={(packet) => { status = `Generated records request: ${packet.court_name} (${packet.records_requested.length} items)`; }}
        onPacketGenerated={(packet) => { status = `Generated factual packet for ${packet.issue_id}:\n${packet.notice}`; }} />
    {:else if tab === "service"}
      <section class="space-y-4"><ServiceCertCard caseId={caseId} profile={profile} defaultMotionTitle={activeFilingTitle || profile.title} /></section>
    {:else if tab === "parties"}
      <CasePartiesTab bind:parties />
    {:else if tab === "documents"}
      <CaseDocumentsTab bind:profile guided={guided} artifacts={artifacts} busy={busy} bind:mdDraft bind:selectedFormPath bind:formInspection bind:formFillResult fillingForm={fillingForm} onfillsection={fillSection} onexportkind={exportKind} onexportpacket={exportPacket} oninspectform={inspectSelectedForm} onfillform={fillSelectedForm} />
    {:else if tab === "evidence"}
      <CaseEvidenceTab profile={profile} artifacts={artifacts} bind:assemblingPacket bind:batesPrefix bind:batesStart bind:exhibitPacketReport onaddevidence={addEvidence} ongeneratepacket={generateExhibitPacket} />
    {:else if tab === "timeline"}
      {#if litigation}<details class="panel p-5"><summary class="font-medium">Procedural paths & deadline tools</summary><div class="mt-4"><ProceduralGraphCard {caseId} bind:profile onOpenWizard={() => showIntakeWizard = true} onOpenExtension={() => showConsultationModal = true} onProfileSaved={load} /></div></details>{/if}
      <CaseTimelineTab bind:profile bind:eventTitle bind:eventDate bind:eventNotes onaddevent={addTimeline} onremoveevent={removeTimeline} />
    {:else if tab === "check"}
      <PreFlightAuditCard {caseId} />
      <CaseCheckTab bind:deadlinePack bind:deadlineRule bind:triggerDate bind:gapTitle bind:gapHard exhibits={exhibits} checkReport={checkReport} onrunchecks={runChecks} onexportics={exportIcs} onexportpacket={exportPacket} onlinkfact={linkFact} ontogglesignoff={toggleSignoff} onaddgap={addGap} />
    {:else if tab === "files"}
      <CaseFilesTab artifacts={artifacts} />
    {:else if tab === "tools"}
      <section class="panel p-5 space-y-4"><h2 class="font-semibold">Matter tools</h2><div class="tool-grid">
        <button class="button" onclick={() => showIntakeWizard = true}>Guided intake</button>
        <button class="button" onclick={() => decodeOrderForCase()}>Read a court order</button>
        <button class="button" onclick={() => showRecordRetrievalModal = true}>Request records</button>
        <button class="button" onclick={() => { preselectedIssueId = null; showAccountabilityModal = true; }}>Accountability routes</button>
        <button class="button" onclick={() => showAdaModal = true}>Accommodation petition</button>
        <button class="button" onclick={() => showWorkflowModal = true}>Workflow progress</button>
        <button class="button" onclick={() => showConsultationModal = true}>Consultation dossier</button>
        <button class="button" onclick={() => showPdfToolsModal = true}>PDF tools</button>
        <button class="button" onclick={() => showSignatureModal = true}>Signature</button>
        <button class="button" onclick={validate}>Validate document</button>
      </div></section>
      {#if profile.mode === 'confidential'}<QuiTamModeBanner {profile} onOpenGhostWizard={() => showGhostWizard = true} onOpenDisclosureExport={disclosure} />{/if}
      <section class="panel p-5 space-y-4"><h2 class="font-semibold">Storage & privacy</h2><p class="text-sm text-muted-foreground break-all">Matter ID: {profile.id}</p><span class="badge">{profile.mode === 'confidential' ? 'Report Fraud / Qui Tam · Confidential Vault' : 'Standard workspace'}</span>
        {#if profile.mode === 'standard' && !profile.sealed_at}<div class="toolbar"><button class="button" onclick={() => showCloudModal = true}>JustLegal.me Cloud Sync</button></div><details><summary class="text-sm">Convert to Report Fraud / Qui Tam (Confidential Vault)</summary><p class="description text-xs">Unlock the vault first. Converting seals this matter locally and permanently blocks cloud sync.</p><label class="block mt-3 text-sm">Vault password<input type="password" bind:value={sealPassword} class="field mt-2" autocomplete="off" /></label><button class="button mt-3" onclick={seal} disabled={busy}>Convert & seal matter</button></details>{/if}
      </section>
    {:else}
      <QuiTamModeBanner {profile} onOpenGhostWizard={() => showGhostWizard = true} onOpenDisclosureExport={disclosure} />
      <CaseConfidentialTab sessionUnlocked={session.unlocked} bind:targetLookup targetHits={targetHits} onlookuptarget={lookupTarget} ondisclosure={disclosure} onexportosb={exportOsb} />
    {/if}

    {/key}
    </div>
    </div>

    {#if report && !report.ok}
      <ul class="text-sm text-destructive list-disc pl-5 space-y-1">{#each report.issues as issue}<li>{issue.field}: {issue.message}</li>{/each}</ul>
    {/if}
    {#if status}<pre class="text-xs whitespace-pre-wrap bg-muted/20 border rounded p-3">{status}</pre>{/if}
    {#if error}<p class="text-sm text-destructive">{error}</p>{/if}
    {#if exportPaths}
      <div class="text-sm border border-emerald-500/30 bg-emerald-500/10 rounded-md p-3 space-y-1">
        <div class="flex items-center justify-between">
          <p class="font-medium text-emerald-700">Export finished</p>
          <button onclick={() => { postExportFilePath = exportPaths?.pdf || exportPaths?.docx || ""; postExportTitle = activeFilingTitle || profile?.title || "Exported Filing"; showPostExportModal = true; }} class="button">Next Steps & Unbundled Review</button>
        </div>
        <p class="font-mono text-xs">{exportPaths.docx}</p>
        <p class="font-mono text-xs">{exportPaths.pdf}</p>
      </div>
    {/if}

    <CaseModals {caseId} bind:profile bind:showCloudModal bind:showPdfToolsModal bind:showSignatureModal bind:showIntakeWizard bind:showAccountabilityModal bind:showAdaModal bind:showRecordRetrievalModal bind:showWorkflowModal bind:showConsultationModal bind:confirmSeal {preselectedIssueId} onReload={load} onConfirmSeal={doSeal} onCloseCloud={() => (showCloudModal = false)} onClosePdf={() => (showPdfToolsModal = false)} onCloseSignature={() => (showSignatureModal = false)} onCloseIntake={() => (showIntakeWizard = false)} onCloseAccountability={() => (showAccountabilityModal = false)} onCloseAda={() => (showAdaModal = false)} onCloseRecordRetrieval={() => (showRecordRetrievalModal = false)} onCloseWorkflow={() => (showWorkflowModal = false)} onCloseConsultation={() => (showConsultationModal = false)} onCancelSeal={() => (confirmSeal = false)} />
    <GhostFacilityWizardModal {caseId} bind:isOpen={showGhostWizard} onClose={() => (showGhostWizard = false)} />
    <PostExportNextStepsModal bind:isOpen={showPostExportModal} filePath={postExportFilePath} documentTitle={postExportTitle} onClose={() => (showPostExportModal = false)} />
  {/if}
</main>

<style>
  .matter-layout { display: grid; grid-template-columns: 160px minmax(0, 1fr); gap: 24px; align-items: start; }
  .matter-navigation { display: grid; gap: 3px; position: sticky; top: 20px; }
  .matter-navigation button { padding: 10px 12px; text-align: left; border-radius: 6px; color: hsl(var(--muted-foreground)); font-size: 13px; }
  .matter-navigation button.chosen { background: #e1eaf7; color: hsl(var(--primary)); font-weight: 600; }
  .matter-navigation button:hover { background: hsl(var(--secondary)); }
  .matter-body { min-width: 0; display: grid; gap: 18px; }
  .overview-stats { display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; }
  .overview-stats button { display: grid; gap: 6px; padding: 16px; background: hsl(var(--background)); border-radius: 7px; text-align: left; }
  .overview-stats strong { font-size: 24px; font-weight: 500; }
  .overview-stats span { font-size: 12px; color: hsl(var(--muted-foreground)); }
  .tool-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 10px; }
  @media (max-width: 1000px) { .matter-layout { grid-template-columns: 1fr; } .matter-navigation { display: flex; flex-wrap: wrap; position: static; border-bottom: 1px solid hsl(var(--border)); padding-bottom: 12px; } }
  @media (max-width: 600px) { .overview-stats { grid-template-columns: 1fr; } .tool-grid { grid-template-columns: 1fr; } }
</style>
