function divideSafely(a: number, b: number): number {
    if (b === 0) return NaN;
    return a / b;
}

console.log(divideSafely(10, 0) + "");
console.log(divideSafely(Infinity, 2) + "");
console.log(divideSafely(NaN, 1) + "");

let arr = [1, 2];
console.log((arr[999] === undefined) + "");
console.log((arr[-1] === undefined) + "");
