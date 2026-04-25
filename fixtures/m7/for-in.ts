let obj = { a: 1, b: 2 };
let sum = 0;
for (let k in obj) {
    if (k === "a") {
        sum = sum + 1;
    }
    if (k === "b") {
        sum = sum + 2;
    }
}
console.log(sum);
