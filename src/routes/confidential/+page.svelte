<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import LegalAirlock from "$lib/components/LegalAirlock.svelte";
  import VaultLauncher from "$lib/components/VaultLauncher.svelte";
  import { ipc } from "$lib/ipc";
  import { session } from "$lib/session.svelte";

  let stage = $state<"airlock" | "auth" | "done">("airlock");

  onMount(async () => {
    try {
      const locked = await ipc<boolean>("is_locked");
      if (!locked) {
        session.unlocked = true;
        session.airlockPassed = true;
        goto("/");
        return;
      }
    } catch {
      // stay on the airlock
    }
    if (session.airlockPassed) {
      stage = "auth";
    }
  });

  function passedAirlock() {
    session.airlockPassed = true;
    stage = "auth";
  }

  function unlocked() {
    session.unlocked = true;
    goto("/");
  }
</script>

<main class="page-content" style="max-width: 850px">
  <button onclick={() => goto("/")} class="text-sm text-muted-foreground hover:text-foreground">Back to workspace</button>
  <div>
    <p class="eyebrow">EO 14395 · False Claims Act & Whistleblower Vault</p>
    <h1 class="text-2xl font-bold mt-1">Report Fraud / Qui Tam</h1>
    <p class="text-sm text-muted-foreground mt-2">
      Prepare whistleblower disclosures, verify providers, and organize Rule 9(b) evidence in an airgapped, locally encrypted vault. Confidentiality protects your First-to-File priority before consulting qui tam counsel or submitting to oversight agencies.
    </p>
  </div>
  {#if stage === "airlock"}
    <LegalAirlock onUnlock={passedAirlock} />
  {:else if stage === "auth"}
    <VaultLauncher onLaunch={unlocked} />
  {/if}
</main>
