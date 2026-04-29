const fact = n => n === 1 || n * fact(n - 1);

console.log(fact(4));
