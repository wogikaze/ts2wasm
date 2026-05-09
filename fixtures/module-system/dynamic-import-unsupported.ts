// Dynamic import — should produce unsupported diagnostic
async function load() {
  const mod = await import("./nonexistent");
  console.log(mod);
}
load();
