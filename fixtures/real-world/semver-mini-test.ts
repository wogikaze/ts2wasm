import { parse, valid, compare } from "./semver-lib";

const p1 = parse("1.2.3");
console.log("p1: " + (p1 ? "ok" : "null"));
