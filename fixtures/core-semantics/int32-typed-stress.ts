function abs(n: number): number {
  if (n < 0) {
    return -n;
  } else {
    return n;
  }
}

function gcd(a: number, b: number): number {
  let x: number = abs(a);
  let y: number = abs(b);
  while (y !== 0) {
    let t: number = x % y;
    x = y;
    y = t;
  }
  return x;
}

function fib(n: number): number {
  if (n <= 1) {
    return n;
  }
  let a: number = 0;
  let b: number = 1;
  let i: number = 2;
  while (i <= n) {
    let c: number = a + b;
    a = b;
    b = c;
    i = i + 1;
  }
  return b;
}

function factMod(n: number, mod: number): number {
  let acc: number = 1;
  for (let i: number = 2; i <= n; i = i + 1) {
    acc = (acc * i) % mod;
  }
  return acc;
}

function branchScore(n: number): number {
  let score: number = 0;
  for (let i: number = 1; i <= n; i = i + 1) {
    if ((i % 15) === 0) {
      score = score + i * 5;
    } else if ((i % 5) === 0) {
      score = score + i * 3;
    } else if ((i % 3) === 0) {
      score = score + i * 2;
    } else {
      score = score + i;
    }
  }
  return score;
}

function checksum(limit: number): number {
  let total: number = 0;
  for (let row: number = 1; row <= limit; row = row + 1) {
    total = total + fib(row);
    total = total + factMod(row + 3, 997);
    total = total + gcd(row * 17, row + 5);
  }
  return total;
}

const LIMIT: number = 18;
let total: number = 0;
let checkpoints: number = 0;

console.log("typed stress start");
console.log("limit " + LIMIT);

for (let i: number = 1; i <= LIMIT; i = i + 1) {
  total = total + checksum(i);
  if ((i % 6) === 0) {
    checkpoints = checkpoints + 1;
    console.log("checkpoint " + i + " " + total);
  }
}

let divisor: number = gcd(total, 2310);
let branch: number = branchScore(40);
let truthy: boolean = (divisor !== 0) && (branch >= 1000);

console.log("total " + total);
console.log("gcd-total-2310 " + divisor);
console.log("branch " + branch);
console.log("truthy-check " + truthy);
console.log("final " + (total + divisor + branch + checkpoints));
console.log("typed stress done");
