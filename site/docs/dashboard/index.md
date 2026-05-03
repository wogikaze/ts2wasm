# Coverage Dashboard

<script>
(function() {
  const pathname = window.location.pathname;
  const base = pathname.replace(/\/?dashboard\/?$/, '');
  const normalizedBase = base === '' ? '' : (base.endsWith('/') ? base : `${base}/`);
  const target = `${normalizedBase}dashboard/index.html`;
  if (window.location.pathname !== target) {
    window.location.replace(target);
  }
})();
</script>

<noscript>
  Redirect failed because JavaScript is disabled.
  <a href="/dashboard/index.html">Open Coverage Dashboard</a>.
</noscript>
