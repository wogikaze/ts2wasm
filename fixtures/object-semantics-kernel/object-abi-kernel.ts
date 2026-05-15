// Object ABI kernel invariants: verifies core object descriptor / property
// behavior expected to work in the current runtime ABI layer.
//
// Unsupported descriptor behaviors are documented in docs/14-runtime-abi.md.

// --- own string-key property read/write ---
let obj: any = {};
obj["key1"] = "value1";
console.log(obj["key1"]);

// --- computed property key read/write ---
let computedKey: string = "dynKey";
obj[computedKey] = 42;
console.log(obj[computedKey]);

// --- non-enumerable descriptor exclusion from Object.keys ---
let enumObj: any = {};
Object.defineProperty(enumObj, "visible", {
  value: "yes",
  enumerable: true,
  writable: false,
  configurable: false,
});
Object.defineProperty(enumObj, "hidden", {
  value: "no",
  enumerable: false,
  writable: false,
  configurable: false,
});
let keys: string[] = Object.keys(enumObj);
console.log(keys.length === 1 && keys[0] === "visible" ? "pass" : "fail");

// --- writable=false assignment (silent in non-strict) ---
let frozen: any = {};
Object.defineProperty(frozen, "ro", {
  value: 99,
  writable: false,
  configurable: false,
  enumerable: true,
});
frozen["ro"] = 999; // silent no-op in non-strict
console.log(frozen["ro"] === 99 ? "pass" : "fail");

// --- configurable=false delete (silent false) ---
let nonConfig: any = {};
Object.defineProperty(nonConfig, "locked", {
  value: "fixed",
  writable: false,
  configurable: false,
  enumerable: true,
});
let deleted: boolean = delete nonConfig["locked"];
console.log(deleted === false ? "pass" : "fail");

// --- prototype pointer read ---
let proto: any = { parentProp: "fromProto" };
let child: any = {};
Object.setPrototypeOf(child, proto);
console.log(child["parentProp"]);
