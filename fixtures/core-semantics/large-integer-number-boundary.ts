const powers: number[] = [];

for (let i = 0; 2 ** i <= 1000000000; i++) {
    powers.push(2 ** i);
}

const mapped = ["1", "536870912", "819264512", "536870912"].map(n => +n);
const seen = new Set<number>();

for (let i = 0; i < mapped.length; i++) {
    seen.add(mapped[i]);
}

const values = [...seen];
values.sort((a, b) => a - b);

console.log(powers[powers.length - 1]);
console.log(values[1]);
console.log(values[2]);
