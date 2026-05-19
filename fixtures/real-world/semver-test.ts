import { parse, valid, compare, gt, lt, eq, satisfies, maxSatisfying, parseOpRange } from "./semver-lib";

console.log("=== semver tests ===");
const p1 = parse("1.2.3");
console.log("1: " + (p1 ? "parsed" : "null"));
const p2 = parse("abc");
console.log("2: " + (p2 ? "parsed" : "null"));
console.log("3: " + (p1 ? String(p1.major) : "fail"));
console.log("4: " + (p1 ? String(p1.minor) : "fail"));
console.log("5: " + (p1 ? String(p1.patch) : "fail"));
const pa = parse("1.0.0-alpha");
console.log("6: " + (pa ? String(pa.prerelease.length) : "fail"));
const pb = parse("1.0.0+sha.abc");
console.log("7: " + (pb ? pb.build[0] : "fail"));
console.log("8: " + String(compare("1.0.0", "2.0.0")));
console.log("9: " + String(compare("1.0.0", "1.0.0")));
console.log("10: " + String(gt("2.0.0", "1.0.0")));
console.log("11: " + String(lt("1.0.0", "1.1.0")));
console.log("12: " + String(compare("1.0.0-alpha", "1.0.0")));
