// Promise minimal substrate: constructor and static resolve
const p = new Promise(() => {});
console.log("created");

const p2 = Promise.resolve(42);
console.log("resolved");

const p3 = Promise.resolve("hello");
console.log("done");
