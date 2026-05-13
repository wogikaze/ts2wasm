let grouped = Map.groupBy([1, 2, 3, 4], value => value % 2 === 0 ? "even" : "odd");
console.log(grouped.get("odd").length);
console.log(grouped.get("odd")[0]);
console.log(grouped.get("even").length);
console.log(grouped.get("even")[1]);

let byLength = Map.groupBy(["ant", "bear", "cat"], value => value.length);
console.log(byLength.size);
console.log(byLength.get(3)[1]);

let byIndex = Map.groupBy(["a", "b", "c"], (_value, index) => index % 2);
console.log(byIndex.get(0).length);
console.log(byIndex.get(1)[0]);
