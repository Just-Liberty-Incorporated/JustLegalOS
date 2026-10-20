<script lang="ts">
  import { fade, scale } from 'svelte/transition';
  import { invoke } from "@tauri-apps/api/core";
  import PromptDialog from "$lib/components/PromptDialog.svelte";
  import { Scale, Shield, AlertTriangle, BookOpen, ExternalLink, Trash2 } from "lucide-svelte";

  let { isOpen = $bindable(false) }: { isOpen?: boolean } = $props();
  let message = $state("");
  let purgeStep = $state<"off" | "confirm" | "password">("off");

  function close() {
    isOpen = false;
  }

  function triggerPurge() {
    message = "";
    purgeStep = "confirm";
  }

  async function openExternal(url: string) {
    try {
      const { openUrl } = await import("@tauri-apps/plugin-opener");
      await openUrl(url);
    } catch {
      window.open(url, "_blank", "noopener,noreferrer");
    }
  }

  async function afterPurgePrompt(value: string) {
    if (purgeStep === "confirm") {
      if (value !== "PURGE") {
        message = "Purge cancelled.";
        purgeStep = "off";
        return;
      }
      purgeStep = "password";
      return;
    }
    purgeStep = "off";
    try {
      const result = await invoke<string>("purge_vault_cache", {
        password: value && value.length ? value : null,
        includeSealed: !!(value && value.length),
      });
      message = result;
      close();
      window.location.reload();
    } catch (e) {
      message = "Failed to delete hunts: " + e;
    }
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6" transition:fade={{ duration: 200 }}>
    <!-- Backdrop -->
    <div 
      class="absolute inset-0 bg-background/90 backdrop-blur-sm" 
      onclick={close}
      aria-hidden="true"
    ></div>

    <!-- Modal Content -->
    <div 
      class="relative w-full max-w-4xl max-h-[92vh] flex flex-col bg-card border rounded-2xl shadow-2xl overflow-hidden"
      transition:scale={{ start: 0.95, duration: 200 }}
      role="dialog"
      aria-modal="true"
      aria-labelledby="help-guide-title"
    >
      <!-- Header -->
      <div class="flex items-center justify-between p-6 border-b shrink-0">
        <div class="flex items-center gap-3">
          <div class="p-2.5 rounded-xl bg-primary/10 text-primary border border-primary/20">
            <BookOpen size={22} />
          </div>
          <div>
            <h2 id="help-guide-title" class="text-xl font-bold">JustLegal OS Educational Guide & Legal Framework</h2>
            <p class="text-xs text-muted-foreground mt-0.5">Deterministic Procedural Architecture, Sovereign Immunity & Evidence Standards</p>
          </div>
        </div>
        <button onclick={close} class="p-2 hover:bg-accent rounded-full transition-colors text-muted-foreground hover:text-foreground">
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
        </button>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-6 space-y-8 text-sm leading-relaxed">
        
        <!-- Disclaimer Banner -->
        <div class="p-4 bg-destructive/10 border border-destructive/20 rounded-xl text-destructive dark:text-red-400 text-xs font-medium space-y-2">
          <p class="font-bold flex items-center gap-2 text-sm">
            <AlertTriangle size={16} />
            IMPORTANT NOTICE & TERMS OF USE
          </p>
          <p>
            JustLegal OS provides <strong>deterministic document assembly, deadline tolling math, and evidentiary organization tools</strong>. 
            It is developed by Just Liberty Inc., a Colorado 501(c)(3) nonprofit. 
            <strong>This software is NOT legal advice, does NOT practice law, and NEVER files or serves documents.</strong>
          </p>
          <p>
            You (or your attorney) remain solely responsible for verifying every citation, fact, and rule before signing and filing. 
            Consult an experienced, licensed attorney for legal advice on your specific matter.
          </p>
        </div>

        <!-- Section 1: Street-Level vs. Systemic Institutional Fraud -->
        <section class="space-y-3">
          <h3 class="text-base font-bold flex items-center gap-2.5 text-foreground">
            <span class="flex items-center justify-center w-6 h-6 rounded-full bg-primary/10 text-primary text-xs font-bold">1</span>
            Two Halves of the Fraud Universe: Street-Level vs. Systemic Contractor Fraud
          </h3>
          <div class="ml-8 space-y-3 text-muted-foreground text-xs leading-relaxed">
            <p>
              Citizens experience fraud in two completely distinct environments, each requiring fundamentally different evidentiary methods:
            </p>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
              <div class="p-3.5 rounded-xl border bg-muted/20 space-y-1.5">
                <strong class="text-foreground text-xs block">Street-Level Storefront Fraud</strong>
                <p>
                  Physical "ghost" clinics, adult daycare centers, and strip-mall shell companies that bill Medicare/Medicaid while shuttered. 
                  Evidence consists of exterior site photos, dark windows during posted hours, empty parking lots, and 10-digit NPI registry checks.
                </p>
              </div>
              <div class="p-3.5 rounded-xl border bg-muted/20 space-y-1.5">
                <strong class="text-foreground text-xs block">Systemic Institutional & Contractor Fraud</strong>
                <p>
                  Government agency mismanagement, automated benefit denials (e.g., state unemployment integrity freezes), and mega-vendor software defects (Deloitte, Maximus). 
                  Evidence consists of prime vendor contracts, SOC 1 Type II audit reports, state open records requests, and data logs.
                </p>
              </div>
            </div>
          </div>
        </section>

        <!-- Section 2: Overcoming Sovereign Immunity & The Stevens Rule -->
        <section class="space-y-3">
          <h3 class="text-base font-bold flex items-center gap-2.5 text-foreground">
            <span class="flex items-center justify-center w-6 h-6 rounded-full bg-primary/10 text-primary text-xs font-bold">2</span>
            Navigating Sovereign Immunity: The Stevens Rule & Valid Legal Bypass Pathways
          </h3>
          <div class="ml-8 space-y-3 text-muted-foreground text-xs leading-relaxed">
            <p>
              Under <em>Vermont Agency of Natural Resources v. United States ex rel. Stevens</em>, 529 U.S. 765 (2000), a state or state agency is not a "person" subject to qui tam liability under the federal False Claims Act. If you name a state agency in an FCA suit, it will be dismissed immediately.
            </p>
            <p>
              However, the law provides <strong>5 valid, constitutional bypass pathways</strong> to achieve accountability:
            </p>
            <ul class="list-disc pl-5 space-y-2">
              <li>
                <strong class="text-foreground">Ex parte Young Doctrine (209 U.S. 123):</strong> You can sue the state agency Executive Director in their official capacity in federal court to enjoin ongoing constitutional or statutory violations (prospective declaratory and injunctive relief).
              </li>
              <li>
                <strong class="text-foreground">ADA Title II Abrogation (<em>United States v. Georgia</em>, 546 U.S. 151 (2006)):</strong> Congress validly abrogated state Eleventh Amendment sovereign immunity for ADA Title II violations that independently violate Fourteenth Amendment Due Process (e.g. inaccessible hearings, automated deprivation). <strong>Money damages and injunctive relief are viable directly against the agency.</strong>
              </li>
              <li>
                <strong class="text-foreground">Rehabilitation Act § 504 Spending Clause Waiver (29 U.S.C. § 794; 42 U.S.C. § 2000d-7):</strong> By accepting federal financial assistance (e.g. DOL Assistance Listing Number 17.225), state departments unambiguously waive sovereign immunity in federal court.
              </li>
              <li>
                <strong class="text-foreground">Private Prime Contractor Liability:</strong> Private tech vendors, billing managers, and corporate contractors (e.g. Deloitte Consulting LLP) have zero sovereign immunity and can be sued for treble damages under 31 U.S.C. § 3729(b)(2)(A)(ii).
              </li>
              <li>
                <strong class="text-foreground">Monell County / Municipal Liability (436 U.S. 658):</strong> Counties, municipalities, and local sheriff/clerk departments possess zero Eleventh Amendment sovereign immunity.
              </li>
            </ul>
          </div>
        </section>

        <!-- Section 3: FRCP Rule 20 Permissive Joinder (Pro Se Multi-Plaintiff Power) -->
        <section class="space-y-3">
          <h3 class="text-base font-bold flex items-center gap-2.5 text-foreground">
            <span class="flex items-center justify-center w-6 h-6 rounded-full bg-primary/10 text-primary text-xs font-bold">3</span>
            Collective Action Without Rule 23: The Pro Se Multi-Plaintiff Joinder Lifeline
          </h3>
          <div class="ml-8 space-y-3 text-muted-foreground text-xs leading-relaxed">
            <p>
              Under <em>Oxendine v. Williams</em>, 509 F.2d 1405 (4th Cir. 1975) and uniform federal doctrine, self-represented (pro se) litigants <strong>cannot represent a class</strong> under Federal Rule of Civil Procedure 23 because a non-lawyer cannot safeguard absent class members' legal rights.
            </p>
            <p>
              JustLegal OS provides the lawful alternative: <strong>FRCP Rule 20(a)(1) Permissive Joinder</strong>.
            </p>
            <ul class="list-disc pl-5 space-y-1.5">
              <li>
                <strong class="text-foreground">Single Docket, Shared Fee:</strong> Between 2 and 25 similarly harmed individuals can file jointly as named co-plaintiffs on a single complaint, splitting one $405 court filing fee.
              </li>
              <li>
                <strong class="text-foreground">Individual Signatures:</strong> Every co-plaintiff signs on their own behalf (bypassing the pro se class ban).
              </li>
              <li>
                <strong class="text-foreground">Mootness Proof:</strong> When the state agency attempts to "pick off" the lead plaintiff by settling or unfreezing their individual account, having co-plaintiffs with active unresolved holds preserves live federal jurisdiction.
              </li>
            </ul>
          </div>
        </section>

        <!-- Section 4: Vernacular-to-Doctrine Translator -->
        <section class="space-y-3">
          <h3 class="text-base font-bold flex items-center gap-2.5 text-foreground">
            <span class="flex items-center justify-center w-6 h-6 rounded-full bg-primary/10 text-primary text-xs font-bold">4</span>
            The Vernacular-to-Doctrine Engine & Mountain-of-Proof Blueprint
          </h3>
          <div class="ml-8 space-y-3 text-muted-foreground text-xs leading-relaxed">
            <p>
              Judges routinely dismiss pro se lawsuits because litigants describe grievances in everyday colloquial language ("they committed treason", "they took kickbacks", "they made the whole town sick") instead of identifying statutory causes of action.
            </p>
            <p>
              JustLegal OS features the <strong>Claim Translator (P17)</strong>, which converts vernacular complaints into exact causes of action and outlines the required <em>Mountain of Proof (Element X + Element Y + Verified Evidence Z)</em>:
            </p>
            <ul class="list-disc pl-5 space-y-1.5">
              <li>"Treason" ➔ 42 U.S.C. § 1983 Due Process / 18 U.S.C. § 666 Federal Program Fraud</li>
              <li>"Kickbacks" ➔ Civil RICO (18 U.S.C. § 1964(c)) & Anti-Kickback Act (41 U.S.C. § 8702)</li>
              <li>"Made Everyone Sick" ➔ RCRA Endangerment (42 U.S.C. § 6972(a)(1)(B)) & State Tort Claims</li>
              <li>"Amend the Constitution" ➔ APA § 553(e) Formal Administrative Rulemaking Petitions</li>
            </ul>
          </div>
        </section>

        <!-- Section 5: Pre-Filing Statutory Open Records Suite -->
        <section class="space-y-3">
          <h3 class="text-base font-bold flex items-center gap-2.5 text-foreground">
            <span class="flex items-center justify-center w-6 h-6 rounded-full bg-primary/10 text-primary text-xs font-bold">5</span>
            Pre-Filing Open Records Suite (CORA, FOIA, NC DES & 501(c)(3) Fee Waivers)
          </h3>
          <div class="ml-8 space-y-3 text-muted-foreground text-xs leading-relaxed">
            <p>
              You cannot prove institutional fraud or systemic non-compliance without the underlying contracts and audit logs. JustLegal OS includes a complete open records suite:
            </p>
            <ul class="list-disc pl-5 space-y-1.5">
              <li>
                <strong class="text-foreground">Governing Regime Finder:</strong> Automatically identifies state sunshine laws (Colorado CORA C.R.S. § 24-72-201, Federal FOIA 5 U.S.C. § 552, NC DES G.S. § 96-4(x), 20 C.F.R. § 603.5(d)).
              </li>
              <li>
                <strong class="text-foreground">Working-Day Calendar Tracking:</strong> Tracks strict statutory response windows (e.g. CORA 3-working-day rule) skipping weekends and legal holidays.
              </li>
              <li>
                <strong class="text-foreground">501(c)(3) Fee Waiver Letters:</strong> Automatically drafts fee waiver requests on behalf of public-interest research sponsored by Just Liberty Inc. (EIN 93-3704483).
              </li>
            </ul>
          </div>
        </section>

        <!-- Section 6: False Claims Act & 60-Day In Camera Seal -->
        <section class="space-y-3">
          <h3 class="text-base font-bold flex items-center gap-2.5 text-foreground">
            <span class="flex items-center justify-center w-6 h-6 rounded-full bg-primary/10 text-primary text-xs font-bold">6</span>
            Qui Tam False Claims Act & The Mandatory Court Seal
          </h3>
          <div class="ml-8 space-y-3 text-muted-foreground text-xs leading-relaxed">
            <p>
              Under 31 U.S.C. § 3730(b)(2), a qui tam complaint <strong>must be filed in camera and remain under seal for at least 60 days</strong> while the Department of Justice investigates.
            </p>
            <div class="p-3.5 rounded-xl border border-amber-500/30 bg-amber-500/10 text-amber-900 dark:text-amber-200">
              <strong class="block mb-1 font-semibold">Strict Airgap & No Social Media:</strong>
              Never discuss or post evidence to TikTok, YouTube, X, Reddit, or the press before filing. Any public disclosure forfeits your 15%–30% relator share under 31 U.S.C. § 3730(e)(4) (the Public Disclosure Bar).
            </div>
          </div>
        </section>

        <!-- Section 7: Local Data Hygiene & Purge -->
        <section class="space-y-3">
          <h3 class="text-base font-bold flex items-center gap-2.5 text-destructive">
            <span class="flex items-center justify-center w-6 h-6 rounded-full bg-destructive/10 text-destructive text-xs font-bold">7</span>
            Local Data Hygiene & Cache Purge
          </h3>
          <div class="ml-8 space-y-3 text-muted-foreground text-xs leading-relaxed">
            <p>
              JustLegal OS operates 100% locally on your computer with zero telemetry. To delete local temporary unsealed case hunts from your machine:
            </p>
            <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between p-3.5 border border-destructive/20 bg-destructive/5 rounded-xl gap-4">
              <div class="space-y-1 flex-1">
                <strong class="block text-destructive font-semibold">Delete Local Hunts</strong>
                <p class="text-xs text-muted-foreground">
                  Permanently deletes unsealed research hunts. Encrypted sealed vaults require re-entering the vault password.
                </p>
              </div>
              <button 
                onclick={triggerPurge}
                class="px-4 py-2 bg-destructive text-destructive-foreground hover:bg-destructive/90 rounded-xl font-medium text-xs transition-colors shrink-0 flex items-center gap-1.5"
              >
                <Trash2 size={14} />
                <span>Delete Local Hunts</span>
              </button>
            </div>
          </div>
        </section>

      </div>

      <!-- Footer with Links and Close Button -->
      <div class="p-4 border-t bg-muted/40 flex flex-wrap items-center justify-between gap-4 shrink-0">
        <div class="flex flex-wrap items-center gap-3 text-xs">
          <button
            type="button"
            onclick={() => openExternal("https://justlegal.me/terms")}
            class="text-primary hover:underline flex items-center gap-1 font-medium"
          >
            <span>Terms of Service</span>
            <ExternalLink size={11} />
          </button>
          <span class="text-muted-foreground">·</span>
          <button
            type="button"
            onclick={() => openExternal("https://justlegal.me/privacy")}
            class="text-primary hover:underline flex items-center gap-1 font-medium"
          >
            <span>Privacy Policy</span>
            <ExternalLink size={11} />
          </button>
          <span class="text-muted-foreground">·</span>
          <button
            type="button"
            onclick={() => openExternal("https://justlegal.me/find-legal-help")}
            class="text-muted-foreground hover:text-foreground flex items-center gap-1 font-medium"
          >
            <span>Find Attorney / Legal Aid</span>
            <ExternalLink size={11} />
          </button>
        </div>

        <button 
          onclick={close} 
          class="px-5 py-2 bg-primary text-primary-foreground hover:bg-primary/90 rounded-xl font-semibold text-xs transition-colors"
        >
          Close Guide
        </button>
      </div>
    </div>
  </div>
{/if}

{#if message}
  <p class="fixed bottom-4 right-4 z-[60] text-sm bg-card border border-border rounded-md px-3 py-2 shadow-lg">{message}</p>
{/if}

<PromptDialog
  open={purgeStep !== "off"}
  title={purgeStep === "password" ? "Vault password (optional)" : "Type PURGE to confirm"}
  body={purgeStep === "password"
    ? "To also delete sealed Confidential vaults, enter the vault password. Leave empty to keep sealed cases."
    : "This deletes unsealed local hunts only. Sealed vaults are skipped unless you also type the vault password next."}
  confirmLabel="Continue"
  onsubmit={afterPurgePrompt}
  oncancel={() => { purgeStep = "off"; message = "Purge cancelled."; }}
/>
