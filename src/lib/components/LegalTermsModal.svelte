<script lang="ts">
  import { onMount } from "svelte";
  import { Shield, Scale, ExternalLink } from "lucide-svelte";

  const STORAGE_KEY = "justlegal_legal_terms_accepted_v1";

  let { isOpen = $bindable(false) }: { isOpen?: boolean } = $props();
  let doNotShowAgain = $state(true);

  onMount(() => {
    const accepted = localStorage.getItem(STORAGE_KEY);
    if (!accepted) {
      isOpen = true;
    }
  });

  export function openModal() {
    isOpen = true;
  }

  function handleAccept() {
    if (doNotShowAgain) {
      localStorage.setItem(STORAGE_KEY, "true");
    }
    isOpen = false;
  }

  async function openExternal(url: string) {
    try {
      const { openUrl } = await import("@tauri-apps/plugin-opener");
      await openUrl(url);
    } catch {
      window.open(url, "_blank", "noopener,noreferrer");
    }
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-background/85 backdrop-blur-md">
    <div
      class="relative w-full max-w-2xl rounded-3xl border border-primary/30 bg-card p-6 md:p-8 shadow-2xl space-y-6 text-foreground animate-in fade-in zoom-in-95 duration-200 flex flex-col max-h-[92vh]"
      role="dialog"
      aria-modal="true"
      aria-labelledby="terms-title"
    >
      <!-- Header -->
      <div class="flex items-start gap-4 shrink-0">
        <div class="p-3 rounded-2xl bg-primary/10 text-primary border border-primary/20 text-2xl shrink-0">
          <Scale size={28} />
        </div>
        <div class="space-y-1">
          <h2 id="terms-title" class="text-xl font-bold tracking-tight text-foreground">Important Legal Notice, Disclaimers & Terms of Use</h2>
          <p class="text-xs font-mono text-muted-foreground uppercase tracking-wider">
            Just Liberty 501(c)(3) Nonprofit · JustLegal OS Legal Operating System
          </p>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="space-y-3.5 text-xs leading-relaxed text-muted-foreground border-y border-border/60 py-4 overflow-y-auto pr-2 flex-1">
        <div class="p-3.5 rounded-xl bg-secondary/50 border border-border/80 text-foreground font-medium">
          JustLegal OS is an open-source, deterministic legal operating system and procedural assembly compiler.
          <strong class="text-primary font-semibold block mt-1">This software is NOT legal advice and does NOT practice law or create an attorney-client relationship.</strong>
        </div>

        <div class="space-y-1">
          <h4 class="font-semibold text-foreground text-xs">1. Document Formatting and Assembly Only</h4>
          <p>
            JustLegal OS formats and compiles your own drafts, records, and evidence into court-ready pleadings, motions, and disclosure statements. It does not invent facts, fabricate citations, or formulate strategic legal theories. You are solely responsible for reviewing every page and citation before filing.
          </p>
        </div>

        <div class="space-y-1">
          <h4 class="font-semibold text-foreground text-xs">2. The Software Never Files or Serves</h4>
          <p>
            JustLegal OS will never e-file documents, submit claims to portals, mail filings, or serve opposing parties on your behalf. You (or your retained counsel) remain solely responsible for physically or electronically signing, filing, and serving every paper in compliance with local court rules and statutory deadlines.
          </p>
        </div>

        <div class="space-y-1">
          <h4 class="font-semibold text-foreground text-xs">3. Vernacular vs. Legal Doctrine</h4>
          <p>
            Everyday colloquial complaints (e.g., alleging "treason", "kickbacks", "systemic corruption", or demanding constitutional amendments) do not constitute actionable court pleadings. To survive motions to dismiss, claims must satisfy exact statutory elements (Element X + Element Y + Mountain of Verified Evidence Z). JustLegal OS provides educational doctrine mapping, but never substitutes for case-specific legal strategy.
          </p>
        </div>

        <div class="space-y-1">
          <h4 class="font-semibold text-foreground text-xs">4. Whistleblower Protection, Court Seals & Public Disclosure Bar</h4>
          <p>
            False Claims Act (31 U.S.C. §§ 3729–3733) qui tam actions must be filed under seal <em>in camera</em> by licensed counsel and served exclusively on the Department of Justice. <strong>Posting allegations, photographs, or observations to social media, YouTube, Reddit, or the press destroys your first-to-file priority and triggers the statutory Public Disclosure Bar (31 U.S.C. § 3730(e)(4)), forfeiting relator bounties.</strong>
          </p>
        </div>

        <div class="space-y-1">
          <h4 class="font-semibold text-foreground text-xs">5. Institutional Targets & Sovereign Immunity</h4>
          <p>
            Under <em>Vermont Agency of Natural Resources v. United States ex rel. Stevens</em>, 529 U.S. 765 (2000), state governments and state agencies are immune from FCA qui tam lawsuits. Suits against state action must proceed through established constitutional and statutory pathways (e.g., <em>Ex parte Young</em> prospective injunctive relief, ADA Title II abrogation under <em>United States v. Georgia</em>, 546 U.S. 151, Rehabilitation Act § 504 spending clause waivers, or naming private prime contractors directly).
          </p>
        </div>

        <div class="space-y-1">
          <h4 class="font-semibold text-foreground text-xs">6. 100% Airgapped Confidentiality & Zero Telemetry</h4>
          <p>
            Standard cases reside in your local documents directory. Confidential and Qui Tam cases are encrypted client-side using XChaCha20-Poly1305 and Argon2id. Zero case telemetry, prompts, or documents are transmitted to remote servers or cloud models.
          </p>
        </div>

        <!-- Links to Web Policies -->
        <div class="pt-2 border-t border-border/40 flex flex-wrap gap-4 text-xs">
          <button
            type="button"
            onclick={() => openExternal("https://justlegal.me/terms")}
            class="text-primary hover:underline flex items-center gap-1 font-medium"
          >
            <span>Read Complete Terms of Service</span>
            <ExternalLink size={12} />
          </button>
          <button
            type="button"
            onclick={() => openExternal("https://justlegal.me/privacy")}
            class="text-primary hover:underline flex items-center gap-1 font-medium"
          >
            <span>Read Privacy Policy</span>
            <ExternalLink size={12} />
          </button>
          <button
            type="button"
            onclick={() => openExternal("https://justlegal.me/find-legal-help")}
            class="text-muted-foreground hover:text-foreground flex items-center gap-1 font-medium"
          >
            <span>Find Licensed Legal Aid / Referral</span>
            <ExternalLink size={12} />
          </button>
        </div>
      </div>

      <!-- Controls -->
      <div class="space-y-4 pt-1 shrink-0">
        <label class="flex items-center gap-2.5 text-xs text-foreground cursor-pointer select-none">
          <input
            type="checkbox"
            bind:checked={doNotShowAgain}
            class="h-4 w-4 rounded border-border text-primary focus:ring-primary/40"
          />
          <span>Do not show this acknowledgment again on startup</span>
        </label>

        <div class="flex justify-between items-center gap-3">
          <div class="flex items-center gap-2 text-[11px] text-muted-foreground">
            <Shield size={14} class="text-emerald-500" />
            <span>Airgapped & Zero-Telemetry</span>
          </div>
          <button
            type="button"
            onclick={handleAccept}
            class="px-6 py-2.5 bg-primary hover:opacity-90 text-primary-foreground font-semibold text-xs rounded-xl shadow-md transition-opacity"
          >
            I Understand & Accept Terms
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
