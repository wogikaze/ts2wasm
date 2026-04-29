let fact = n => (n === 1 && 1) || n * fact(n - 1);

fact = 7;
console.log(fact(2));
