/** Read-only refresh on return from an external agent, with periodic visible checks. */
export function observeWorkspace(refresh: () => Promise<unknown>, interval = 8000) {
  let stopped = false;
  let pending = false;
  async function run() {
    if (stopped || pending || document.visibilityState !== 'visible') return;
    pending = true;
    try { await refresh(); } finally { pending = false; }
  }
  const timer = window.setInterval(run, interval);
  window.addEventListener('focus', run);
  document.addEventListener('visibilitychange', run);
  return () => {
    stopped = true;
    window.clearInterval(timer);
    window.removeEventListener('focus', run);
    document.removeEventListener('visibilitychange', run);
  };
}
