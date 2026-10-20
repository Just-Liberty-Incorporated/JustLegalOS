<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";
  import { LayoutGrid, Bot, Files, FileText, Shield, Scale } from "lucide-svelte";
  import HelpMenu from "$lib/components/HelpMenu.svelte";
  import { runningInTauri } from "$lib/ipc";
  let { children } = $props();
  const navigation = [
    { href: "/", label: "Workspace", icon: LayoutGrid },
    { href: "/ai", label: "AI", icon: Bot },
    { href: "/forms", label: "Forms & templates", icon: Files },
    { href: "/draft", label: "Draft formatter", icon: FileText },
    { href: "/confidential", label: "Report Fraud / Qui Tam", icon: Shield },
  ];
</script>

<div class="app-shell">
  <a class="skip-link" href="#workspace-content">Skip to content</a>
  <aside class="app-sidebar">
    <a href="/" class="app-brand"><Scale size={22} strokeWidth={1.6} /><span>JustLegal <strong>OS</strong></span></a>
    <nav aria-label="Main navigation" class="app-navigation">
      {#each navigation as item}
        {@const active = item.href === "/" ? $page.url.pathname === "/" || $page.url.pathname.startsWith("/case/") : $page.url.pathname.startsWith(item.href)}
        <a href={item.href} class:active aria-current={active ? "page" : undefined}><item.icon size={18} strokeWidth={1.7} />{item.label}</a>
      {/each}
    </nav>
    <div class="sidebar-bottom">
      <p>{runningInTauri() ? "Local workspace" : "Browser preview · sample data"}</p>
      <HelpMenu />
    </div>
  </aside>
  <div id="workspace-content" class="workspace-content" tabindex="-1">
    {#key $page.url.pathname}
      {@render children()}
    {/key}
  </div>
</div>
