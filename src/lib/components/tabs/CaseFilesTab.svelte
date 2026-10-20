<script lang="ts">
  import { FileText, FolderOpen, Copy } from 'lucide-svelte';
  import { runningInTauri } from '$lib/ipc';
  import type { CaseArtifact } from '$lib/types/case';
  let { artifacts }: { artifacts: CaseArtifact[] } = $props();
  let filter = $state('');
  let status = $state('');
  let error = $state('');
  async function reveal(path: string) {
    error = '';
    try { const { revealItemInDir } = await import('@tauri-apps/plugin-opener'); await revealItemInDir(path); }
    catch (e) { error = 'Could not show this file. You can copy its path below. ' + String(e); }
  }
  async function copy(path: string) {
    error = '';
    try { await navigator.clipboard.writeText(path); status = 'File path copied.'; }
    catch { error = 'Clipboard access is unavailable. Select the displayed path to copy it.'; }
  }
</script>
<section class="panel">
  <div class="panel-heading"><h2>Files & exports</h2><input class="field" style="max-width: 220px" aria-label="Find a file" placeholder="Find a file" bind:value={filter} /></div>
  {#if error}<p class="notice error m-4" role="alert">{error}</p>{/if}
  {#if status}<p class="text-xs text-primary px-5 pt-3" role="status">{status}</p>{/if}
  {#if artifacts.length === 0}<div class="empty-state"><h3>No files yet</h3><p>Documents added or exported by you and your agent appear here.</p></div>
  {:else}<ul>{#each artifacts.filter(item => (item.name + ' ' + item.kind).toLowerCase().includes(filter.toLowerCase())) as item}<li class="file-row"><FileText size={19} strokeWidth={1.5} /><div class="min-w-0 flex-1"><p class="font-medium text-sm">{item.name} <span class="badge ml-2">{item.kind}</span></p><p class="file-path">{item.path}</p><div class="toolbar mt-2"><button class="button quiet" onclick={() => copy(item.path)}><Copy size={13} /> Copy path</button><button class="button quiet" onclick={() => reveal(item.path)} disabled={!runningInTauri()}><FolderOpen size={14} /> Show in folder</button></div></div></li>{:else}<li class="empty-state">No files match your search.</li>{/each}</ul>{/if}
</section>
<style>
  .file-row { display: flex; gap: 14px; padding: 18px 20px; border-bottom: 1px solid hsl(var(--border)); }
  .file-row:last-child { border-bottom: 0; }
  .file-row > :global(svg) { color: hsl(var(--muted-foreground)); margin-top: 3px; flex-shrink: 0; }
  .file-path { font-size: 12px; color: hsl(var(--muted-foreground)); overflow-wrap: anywhere; margin-top: 7px; user-select: text; }
</style>
