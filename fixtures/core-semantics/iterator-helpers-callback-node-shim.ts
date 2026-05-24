const mapped: any = Iterator.from([1, 2, 3, 4])
  .map((value: number) => value * 2)
  .filter((value: number) => value > 4)
  .toArray();
console.log(mapped.length);
console.log(mapped[0]);
console.log(mapped[1]);

const sum: any = Iterator.from([1, 2, 3]).reduce(
  (acc: number, value: number) => acc + value,
  0,
);
console.log(sum);

const source: any = [1, 2, 3, 4];
const copiedMapped: any = Iterator.from(Array.from(source))
  .map((value: number) => value * 2)
  .filter((value: number) => value > 4)
  .toArray();
console.log(copiedMapped.length);
console.log(copiedMapped[0]);
console.log(copiedMapped[1]);

const copiedSum: any = Iterator.from(Array.from(source))
  .map((value: number) => value * 2)
  .reduce((acc: number, value: number) => acc + value, 0);
console.log(copiedSum);
