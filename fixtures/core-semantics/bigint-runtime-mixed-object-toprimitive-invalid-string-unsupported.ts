console.log(({ toString: () => "not-a-bigint" }) == 1n);
