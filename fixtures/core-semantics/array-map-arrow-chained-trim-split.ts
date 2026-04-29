let inputText = "  a b\nc d\n";
let rows = inputText.trim().split("\n").map(row => row.split(" "));

console.log(rows.length);
console.log(rows[0].length);
console.log(rows[0][0]);
console.log(rows[0][1]);
console.log(rows[1].length);
console.log(rows[1][0]);
console.log(rows[1][1]);
