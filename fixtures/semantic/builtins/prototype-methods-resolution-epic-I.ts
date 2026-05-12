// Epic I-20260513: Prototype method resolution
// Verify that builtin prototype method names resolve correctly.

// Array.prototype methods
let flat = Array.prototype.flat;
let flatMap = Array.prototype.flatMap;

// String.prototype methods
let matchAll = String.prototype.matchAll;

// Object.prototype methods
let hasOwn = Object.prototype.hasOwnProperty;
let toString = Object.prototype.toString;

// Function.prototype methods
let bind = Function.prototype.bind;
let call = Function.prototype.call;
let apply = Function.prototype.apply;

// RegExp.prototype methods
let regexpTest = RegExp.prototype.test;
let regexpExec = RegExp.prototype.exec;

// Promise.prototype methods
let then = Promise.prototype.then;
let _catch = Promise.prototype.catch;
