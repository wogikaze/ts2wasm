// Array.prototype.reduceRight basic test
const arr = [1, 2, 3];
const sum = arr.reduceRight((acc: number, val: number) => acc + val, 0);
console.log(sum);
