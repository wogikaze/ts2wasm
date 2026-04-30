console.log(({ valueOf: () => 1n }) == 1n);
console.log(1n == ({ valueOf: () => 1n }));
console.log(({ valueOf: () => 2n }) != 1n);
