function accessDecision(isAdmin: boolean, isOwner: boolean, locked: boolean): string {
  if (locked && !isAdmin) {
    return "deny";
  }
  if (isAdmin || isOwner) {
    return "allow";
  }
  return "deny";
}

console.log(accessDecision(false, false, false));
console.log(accessDecision(false, true, false));
console.log(accessDecision(false, true, true));
console.log(accessDecision(true, false, true));
