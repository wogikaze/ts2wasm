# Coverage Dashboard

<script>
(function() {
  if (typeof window === 'undefined') return;
  const pathname = window.location.pathname;
  const dashboardStart = pathname.indexOf('/dashboard');
  const rootPrefix = dashboardStart === -1 ? '' : pathname.slice(0, dashboardStart);
  const normalizedPrefix = rootPrefix.endsWith('/') ? rootPrefix.slice(0, -1) : rootPrefix;
  const target = `${normalizedPrefix}/dashboard/index.html`;
  if (window.location.pathname !== target) {
    window.location.replace(target);
  }
})();
</script>

<noscript>
  Redirect failed because JavaScript is disabled.
  <a href="/dashboard/index.html">Open Coverage Dashboard</a>.
</noscript>
