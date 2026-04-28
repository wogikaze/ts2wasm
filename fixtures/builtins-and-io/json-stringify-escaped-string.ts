let obj = { a: "x\"y", b: "c\\d", c: "line\nend", d: ["tab\tend"] };
console.log(JSON.stringify(obj));
console.log(JSON.stringify("quote\"slash\\"));
