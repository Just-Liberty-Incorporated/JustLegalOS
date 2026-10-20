<script lang="ts">
  import { Shield, AlertTriangle, ExternalLink, BookOpen, Lock, Scale, FileText } from "lucide-svelte";
  let { onUnlock }: { onUnlock: () => void } = $props();
</script>

<div class="space-y-6">
  <section class="panel p-6 space-y-5">
    <div class="flex items-center gap-3">
      <div class="p-2.5 rounded-lg bg-blue-500/10 text-blue-600">
        <Shield size={22} strokeWidth={2} />
      </div>
      <div>
        <h2 class="text-lg font-semibold">Confidentiality & Airgap Boundary</h2>
        <p class="text-xs text-muted-foreground">Local XChaCha20-Poly1305 Encrypted Vault · Zero Remote Telemetry</p>
      </div>
    </div>

    <p class="text-sm leading-relaxed text-muted-foreground">
      Matters created in this section are stored strictly on your local computer inside an encrypted vault. They are permanently excluded from cloud sync and remote API access. If you connect an AI assistant, use a fully local offline model (or run commands via the local CLI) to maintain statutory seal boundaries.
    </p>

    <div class="rounded-lg border border-amber-500/30 bg-amber-500/10 p-4 space-y-2 text-xs">
      <div class="flex items-center gap-2 font-medium text-amber-800">
        <AlertTriangle size={15} />
        <span>Mandatory 60-Day Federal Court Seal (31 U.S.C. § 3730(b)(2))</span>
      </div>
      <p class="text-amber-900/90 leading-relaxed">
        Under federal law, a qui tam False Claims Act complaint <strong>must be filed under seal in camera</strong> by a licensed attorney and served on the Department of Justice before any public disclosure. <strong>Never post allegations, photos, or documents to social media, YouTube, Reddit, or press before filing.</strong> Doing so destroys your First-to-File priority, triggers the Public Disclosure Bar (31 U.S.C. § 3730(e)(4)), and forfeits any potential relator bounty.
      </p>
    </div>

    <div class="pt-2">
      <button class="button primary w-full justify-center text-sm py-2.5" onclick={onUnlock}>
        <Lock size={15} /> Continue to Encrypted Vault
      </button>
    </div>
  </section>

  <!-- Educational & Legal Framework Section -->
  <section class="panel p-6 space-y-5">
    <div class="flex items-center justify-between border-b pb-4">
      <div class="flex items-center gap-2.5">
        <BookOpen size={18} class="text-blue-600" />
        <h3 class="font-semibold text-sm">Whistleblower Educational Framework & Responsibilities</h3>
      </div>
      <span class="text-xs text-muted-foreground">Executive Order 14395 Guidance</span>
    </div>

    <div class="grid gap-4 text-xs">
      <!-- 1. The Dual Track Fork -->
      <details class="p-3.5 rounded-lg border bg-muted/20" open>
        <summary class="font-semibold text-foreground cursor-pointer flex items-center justify-between">
          <span>1. The Critical Legal Fork: Track A (Private Counsel) vs. Track B (Agency Tips)</span>
        </summary>
        <div class="mt-3 space-y-2 text-muted-foreground leading-relaxed">
          <p>
            Before submitting your dossier, determine which route matches your situation:
          </p>
          <ul class="list-disc pl-5 space-y-1.5">
            <li>
              <strong class="text-foreground">Track A (Relator Reward Track):</strong> For substantial fraud (typically $1M+) where you have direct, non-public insider documentation. You retain private False Claims Act counsel to file a sealed complaint in federal district court. Relators are statutorily entitled to <strong>15% to 30%</strong> of the government's recovery (31 U.S.C. § 3730(d)).
            </li>
            <li>
              <strong class="text-foreground">Track B (Direct Agency Reporting):</strong> For localized storefront abuse, administrative tips, or if you simply want fraud halted immediately. You submit pre-trimmed narrative blocks directly to agency portals (HHS-OIG, DOJ Civil Fraud, State MFCUs). <em>Notice: Direct agency web tips waive private relator bounties.</em>
            </li>
          </ul>
        </div>
      </details>

      <!-- 2. The 3-Tier Verification Architecture -->
      <details class="p-3.5 rounded-lg border bg-muted/20">
        <summary class="font-semibold text-foreground cursor-pointer flex items-center justify-between">
          <span>2. The 3-Tier Verification Architecture (Avoiding False Claims)</span>
        </summary>
        <div class="mt-3 space-y-2 text-muted-foreground leading-relaxed">
          <p>
            To prevent false accusations and satisfy FRCP Rule 9(b) heightened particularity, JustLegal OS evaluates suspects across three distinct evidentiary tiers:
          </p>
          <ol class="list-decimal pl-5 space-y-1.5">
            <li>
              <strong class="text-foreground">Tier 1 (CMS NPPES Provider Registry):</strong> Cross-references 10-digit NPIs, active registration status, physical practice addresses, and registered taxonomies (e.g., dispatch/telehealth vs. physical facility).
            </li>
            <li>
              <strong class="text-foreground">Tier 2 (USAspending Direct Federal Awards):</strong> Verifies direct procurement contracts, prime grants, and direct loans from HHS, DoD, or VA under Federal Award ID / UEI numbers.
            </li>
            <li>
              <strong class="text-foreground">Tier 3 (State-Administered Medicaid Claims Guidance):</strong> State Medicaid and managed care organization (MCO) funds flow through state health agencies rather than direct USAspending awards. The absence of direct awards on USAspending does not indicate zero Medicaid billing; state FOIA / public records requests are required for state Medicaid claims data.
            </li>
          </ol>
        </div>
      </details>

      <!-- 3. Safe Harbors & Lawful Observation -->
      <details class="p-3.5 rounded-lg border bg-muted/20">
        <summary class="font-semibold text-foreground cursor-pointer flex items-center justify-between">
          <span>3. Lawful Evidence Collection & HIPAA Safe Harbor (45 C.F.R. § 164.502(j))</span>
        </summary>
        <div class="mt-3 space-y-2 text-muted-foreground leading-relaxed">
          <ul class="list-disc pl-5 space-y-1.5">
            <li>
              <strong class="text-foreground">HIPAA Whistleblower Protection:</strong> Healthcare employees and contractors are protected when disclosing Protected Health Information (PHI) <em>only</em> if disclosed directly to retained legal counsel or government oversight officials. Never post patient information publicly.
            </li>
            <li>
              <strong class="text-foreground">Lawful Observation Bounds:</strong> Citizens may lawfully photograph exterior storefronts, dark windows during posted operating hours, or empty parking lots from a public sidewalk or public right-of-way. Do not trespass on private property, break locks, or access unauthorized computer systems.
            </li>
          </ul>
        </div>
      </details>

      <!-- 4. Systemic Contractor Fraud & Sovereign Immunity -->
      <details class="p-3.5 rounded-lg border bg-muted/20">
        <summary class="font-semibold text-foreground cursor-pointer flex items-center justify-between">
          <span>4. Systemic Contractor Fraud & Sovereign Immunity (The Stevens Rule)</span>
        </summary>
        <div class="mt-3 space-y-2 text-muted-foreground leading-relaxed">
          <p>
            Under <em>Vermont ANR v. Stevens</em>, 529 U.S. 765, a state agency cannot be sued in an FCA qui tam case. Do not name state departments in qui tam pleadings. Instead, pivot to:
          </p>
          <ul class="list-disc pl-5 space-y-1.5">
            <li>
              <strong class="text-foreground">Private Prime Contractors:</strong> Private IT and billing contractors (e.g. Deloitte, Maximus) have zero sovereign immunity under 31 U.S.C. § 3729(b)(2)(A)(ii).
            </li>
            <li>
              <strong class="text-foreground">ADA Title II & § 504 Waivers:</strong> Agencies accepting federal grants waive immunity for disability access violations and due process deprivations (<em>United States v. Georgia</em>, 546 U.S. 151; 29 U.S.C. § 794).
            </li>
            <li>
              <strong class="text-foreground">Ex parte Young (209 U.S. 123):</strong> Sue agency directors in official capacity for prospective injunctive and declaratory relief.
            </li>
          </ul>
        </div>
      </details>

      <!-- 5. Pre-Filing Open Records Discovery -->
      <details class="p-3.5 rounded-lg border bg-muted/20">
        <summary class="font-semibold text-foreground cursor-pointer flex items-center justify-between">
          <span>5. Pre-Filing Open Records Suite (CORA, FOIA & 501(c)(3) Fee Waivers)</span>
        </summary>
        <div class="mt-3 space-y-2 text-muted-foreground leading-relaxed">
          <p>
            Before filing, use state open records (CORA, FOIA, NC DES G.S. § 96-4(x), 20 C.F.R. § 603.5(d)) to acquire prime contracts, amendments, and SOC 1 Type II audit reports. JustLegal OS formats formal requests with fee waivers under Just Liberty Inc. (501(c)(3) EIN 93-3704483).
          </p>
        </div>
      </details>
    </div>

    <!-- External Verified Resources & Policies -->
    <div class="pt-2 border-t flex flex-wrap items-center gap-4 text-xs">
      <span class="text-muted-foreground font-medium">Authoritative Resources:</span>
      <a href="https://justlegal.me/qui-tam" target="_blank" rel="noopener noreferrer" class="text-blue-600 hover:underline flex items-center gap-1">
        <span>Fraud Triage Standard</span>
        <ExternalLink size={12} />
      </a>
      <a href="https://justlegal.me/fraud-triage" target="_blank" rel="noopener noreferrer" class="text-blue-600 hover:underline flex items-center gap-1">
        <span>Online NPI Cross-Check</span>
        <ExternalLink size={12} />
      </a>
      <a href="https://taf.org" target="_blank" rel="noopener noreferrer" class="text-blue-600 hover:underline flex items-center gap-1">
        <span>The Anti-Fraud Coalition (TAF)</span>
        <ExternalLink size={12} />
      </a>
      <a href="https://oig.hhs.gov/fraud/report-fraud/" target="_blank" rel="noopener noreferrer" class="text-blue-600 hover:underline flex items-center gap-1">
        <span>HHS-OIG Whistleblower Portal</span>
        <ExternalLink size={12} />
      </a>
      <span class="text-muted-foreground">·</span>
      <a href="https://justlegal.me/terms" target="_blank" rel="noopener noreferrer" class="text-muted-foreground hover:text-foreground hover:underline flex items-center gap-1 font-medium">
        <span>Terms of Service</span>
        <ExternalLink size={12} />
      </a>
      <a href="https://justlegal.me/privacy" target="_blank" rel="noopener noreferrer" class="text-muted-foreground hover:text-foreground hover:underline flex items-center gap-1 font-medium">
        <span>Privacy Policy</span>
        <ExternalLink size={12} />
      </a>
    </div>
  </section>
</div>

