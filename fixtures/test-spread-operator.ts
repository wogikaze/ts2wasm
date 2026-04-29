function sum(a: number, b: number, c: number): number {
    return a + b + c;
}

const result = sum(...[3, 4, 5]);
console.log(result);
