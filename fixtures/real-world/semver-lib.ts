// Minimal semver implementation - parse, compare, valid, sort
export interface SemVer {
  major: number;
  minor: number;
  patch: number;
  prerelease: string[];
  build: string[];
}

export function parse(v: string): SemVer | null {
  // Find first dot
  const dot1 = v.indexOf(".");
  if (dot1 < 0) return null;
  const majorS = v.slice(0, dot1);
  // Find second dot
  const rest1 = v.slice(dot1 + 1);
  const dot2 = rest1.indexOf(".");
  if (dot2 < 0) return null;
  const minorS = rest1.slice(0, dot2);
  // Parse rest for patch, prerelease, build
  const rest2 = rest1.slice(dot2 + 1);
  // Check for prerelease or build
  let patchS = rest2;
  let prerelease: string[] = [];
  let build: string[] = [];
  const dash = rest2.indexOf("-");
  const plus = rest2.indexOf("+");
  if (plus >= 0 && (dash < 0 || plus < dash)) {
    patchS = rest2.slice(0, plus);
    build = rest2.slice(plus + 1).split(".");
  } else if (dash >= 0) {
    patchS = rest2.slice(0, dash);
    const afterDash = rest2.slice(dash + 1);
    const plus2 = afterDash.indexOf("+");
    if (plus2 >= 0) {
      prerelease = afterDash.slice(0, plus2).split(".");
      build = afterDash.slice(plus2 + 1).split(".");
    } else {
      prerelease = afterDash.split(".");
    }
  }
  return {
    major: parseInt(majorS, 10),
    minor: parseInt(minorS, 10),
    patch: parseInt(patchS, 10),
    prerelease: prerelease,
    build: build,
  };
}

export function valid(v: string): string {
  const s = parse(v);
  return s ? v : "";
}

export function compare(a: string, b: string): number {
  const sa = parse(a);
  const sb = parse(b);
  if (!sa || !sb) return !sa ? -1 : 1;
  // Compare major
  if (sa.major < sb.major) return -1;
  if (sa.major > sb.major) return 1;
  // Compare minor
  if (sa.minor < sb.minor) return -1;
  if (sa.minor > sb.minor) return 1;
  // Compare patch
  if (sa.patch < sb.patch) return -1;
  if (sa.patch > sb.patch) return 1;
  // Compare prerelease
  const pa = sa.prerelease;
  const pb = sb.prerelease;
  const len = pa.length > pb.length ? pa.length : pb.length;
  for (let i = 0; i < len; i++) {
    if (i >= pa.length) return -1;
    if (i >= pb.length) return 1;
    if (pa[i] !== pb[i]) {
      const an = parseInt(pa[i], 10);
      const bn = parseInt(pb[i], 10);
      if (!isNaN(an) && !isNaN(bn)) {
        if (an < bn) return -1;
        if (an > bn) return 1;
      }
      if (pa[i] < pb[i]) return -1;
      return 1;
    }
  }
  return 0;
}

export function gt(a: string, b: string): boolean {
  return compare(a, b) > 0;
}

export function lt(a: string, b: string): boolean {
  return compare(a, b) < 0;
}

export function eq(a: string, b: string): boolean {
  return compare(a, b) === 0;
}

export function maxSatisfying(versions: string[], range: string): string {
  let best = "";
  for (let i = 0; i < versions.length; i++) {
    const v = versions[i];
    if (satisfies(v, range)) {
      if (best === "" || compare(v, best) > 0) {
        best = v;
      }
    }
  }
  return best;
}

export function parseOpRange(range: string): string[] {
  const ops = [">=", "<=", "!=", ">", "<", "=", "!"];
  for (let i = 0; i < ops.length; i++) {
    const op = ops[i];
    if (range.slice(0, op.length) === op) {
      const ver = range.slice(op.length).trim();
      if (parse(ver)) return [op, ver];
    }
  }
  if (parse(range)) return ["=", range];
  return [];
}

export function satisfies(version: string, range: string): boolean {
  if (range === "*" || range === "x" || range === "X") return true;
  if (range === version) return true;
  const parts = parseOpRange(range);
  if (parts.length < 2) return false;
  const op = parts[0];
  const target = parts[1];
  if (op === "=") return eq(version, target);
  if (op === ">") return gt(version, target);
  if (op === "<") return lt(version, target);
  if (op === ">=") return compare(version, target) >= 0;
  if (op === "<=") return compare(version, target) <= 0;
  if (op === "!") return !eq(version, target);
  return false;
}
