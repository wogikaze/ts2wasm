let text = "alpha beta beta";

let hit = text.match(/beta/);
console.log(hit === null ? "none" : "" + hit);
console.log(text.match(/gamma/) === null);

console.log(text.replace(/beta/, "B"));
console.log(text.replace(/beta/g, "B"));
console.log("abc123".replace(/\d+/, "#"));
