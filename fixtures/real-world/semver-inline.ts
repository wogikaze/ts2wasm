function parse(v: string): object | null {
  const dot1 = v.indexOf(".");
  if (dot1 < 0) return null;
  const majorS = v.slice(0, dot1);
  const rest1 = v.slice(dot1 + 1);
  const dot2 = rest1.indexOf(".");
  if (dot2 < 0) return null;
  const minorS = rest1.slice(0, dot2);
  const rest2 = rest1.slice(dot2 + 1);
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

const p1 = parse("1.2.3");
console.log("p1: " + (p1 ? "ok" : "null"));
