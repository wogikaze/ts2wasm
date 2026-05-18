let defaults = Function("a = 1", "b", "return a + b");
console.log(defaults.length);
console.log(defaults(undefined, 2));

let leadingDefault = Function("a", "b = 1", "return a + b");
console.log(leadingDefault.length);
console.log(leadingDefault(2));

let destructuring = Function("{a}", "return a");
console.log(destructuring.length);
console.log(destructuring({ a: 4 }));

let constructedRest = new Function("...items", "return items.length");
console.log(constructedRest.length);
console.log(constructedRest(1, 2));
