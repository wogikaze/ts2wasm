function transition(state: string, event: string): string {
  if (state === "idle" && event === "start") {
    return "running";
  }
  if (state === "running" && event === "pause") {
    return "paused";
  }
  if (state === "paused" && event === "resume") {
    return "running";
  }
  if (state === "running" && event === "stop") {
    return "idle";
  }
  return state;
}

console.log(transition("idle", "start"));
console.log(transition("running", "pause"));
console.log(transition("paused", "resume"));
console.log(transition("running", "stop"));
