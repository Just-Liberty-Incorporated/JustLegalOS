<script lang="ts">
  import { goto } from "$app/navigation";
  import { ipc } from "$lib/ipc";
  import ExternalConfirm from "$lib/components/ExternalConfirm.svelte";
  import HelpModal from "$lib/components/HelpModal.svelte";
  import LegalTermsModal from "$lib/components/LegalTermsModal.svelte";
  import type { HelpLink, HelpLinks } from "$lib/types/packs";
  import { BookOpen, Shield, Scale, ExternalLink, HelpCircle } from "lucide-svelte";

  let open = $state(false);
  let confirmOpen = $state(false);
  let confirmText = $state("This will open your web browser to JustLegal's online resources. You need an internet connection. Continue?");
  let pending = $state<HelpLink | null>(null);
  let links = $state<HelpLink[]>([]);
  let helpModalOpen = $state(false);
  let termsModalOpen = $state(false);

  async function load() {
    try {
      const set = await ipc<HelpLinks>("help_links_cmd");
      links = set.links || [];
      confirmText = set.confirm || confirmText;
    } catch {
      links = [];
    }
  }

  function ask(link: HelpLink) {
    pending = link;
    confirmOpen = true;
    open = false;
  }

  function openTermsDirect() {
    open = false;
    ask({
      id: "terms-of-service",
      kind: "policy",
      label: "JustLegal Terms of Service",
      description: "Official terms of service for JustLegal OS and Just Liberty Inc.",
      url: "https://justlegal.me/terms",
      verified: true
    });
  }

  function openPrivacyDirect() {
    open = false;
    ask({
      id: "privacy-policy",
      kind: "policy",
      label: "JustLegal Privacy Policy",
      description: "Official privacy policy and zero-telemetry commitments.",
      url: "https://justlegal.me/privacy",
      verified: true
    });
  }
</script>

<div class="relative">
  <button
    class="text-xs border border-border px-3 py-1.5 rounded-lg hover:bg-accent/20 transition-colors flex items-center gap-1.5 font-medium"
    aria-label="Help & Legal Terms"
    onclick={() => {
      open = !open;
      if (open) load();
    }}
  >
    <HelpCircle size={14} />
    <span>Help & Terms</span>
  </button>

  {#if open}
    <div class="absolute left-0 bottom-full mb-2 w-72 border border-border bg-card rounded-xl shadow-xl z-40 p-2 space-y-1 animate-in fade-in zoom-in-95 duration-150">
      
      <!-- Primary Educational Guide -->
      <button 
        onclick={() => { open = false; helpModalOpen = true; }} 
        class="w-full text-left text-xs px-2.5 py-2 rounded-lg hover:bg-accent/30 font-medium flex items-center gap-2 text-foreground"
      >
        <BookOpen size={14} class="text-primary" />
        <div>
          <div>How It Works & Educational Guide</div>
          <div class="text-[10px] text-muted-foreground font-normal">Sovereign immunity, joinder, open records & fraud</div>
        </div>
      </button>

      <!-- Legal Terms & Disclaimers Modal -->
      <button 
        onclick={() => { open = false; termsModalOpen = true; }} 
        class="w-full text-left text-xs px-2.5 py-2 rounded-lg hover:bg-accent/30 font-medium flex items-center gap-2 text-foreground"
      >
        <Scale size={14} class="text-primary" />
        <div>
          <div>Legal Terms & Disclaimers</div>
          <div class="text-[10px] text-muted-foreground font-normal">Non-legal-advice rules & airgap boundaries</div>
        </div>
      </button>

      <div class="border-t border-border/60 my-1"></div>

      <!-- Navigation shortcuts -->
      <button onclick={() => { open = false; goto("/guide"); }} class="w-full text-left text-xs px-2.5 py-1.5 rounded-md hover:bg-accent/20">
        What do I need to file? (Finder)
      </button>
      <button onclick={() => { open = false; goto("/packs"); }} class="w-full text-left text-xs px-2.5 py-1.5 rounded-md hover:bg-accent/20">
        Forms packs catalog
      </button>
      <button onclick={() => { open = false; goto("/ai"); }} class="w-full text-left text-xs px-2.5 py-1.5 rounded-md hover:bg-accent/20">
        AI briefing & 111 CLI tools
      </button>

      <div class="border-t border-border/60 my-1"></div>

      <!-- Official Web Links -->
      <div class="px-2 pt-1 pb-0.5 text-[10px] uppercase tracking-wider text-muted-foreground font-semibold">
        Policies & External Resources
      </div>
      
      <div class="grid grid-cols-2 gap-1 px-1">
        <button 
          onclick={openTermsDirect} 
          class="text-left text-[11px] px-2 py-1 rounded hover:bg-accent/20 text-primary flex items-center justify-between"
        >
          <span>Terms</span>
          <ExternalLink size={10} />
        </button>
        <button 
          onclick={openPrivacyDirect} 
          class="text-left text-[11px] px-2 py-1 rounded hover:bg-accent/20 text-primary flex items-center justify-between"
        >
          <span>Privacy</span>
          <ExternalLink size={10} />
        </button>
      </div>

      {#each links as link}
        <button onclick={() => ask(link)} class="w-full text-left text-xs px-2.5 py-1.5 rounded hover:bg-accent/20 flex items-center justify-between text-muted-foreground hover:text-foreground">
          <span>{link.label}</span>
          <ExternalLink size={10} />
        </button>
      {/each}

      <div class="border-t border-border/60 my-1"></div>
      <p class="text-[10px] text-muted-foreground px-2 py-1 leading-tight">
        A confirmation dialog appears before opening external links. The app remains 100% usable offline.
      </p>
    </div>
  {/if}
</div>

<ExternalConfirm bind:open={confirmOpen} bind:link={pending} confirm={confirmText} />
<HelpModal bind:isOpen={helpModalOpen} />
<LegalTermsModal bind:isOpen={termsModalOpen} />
