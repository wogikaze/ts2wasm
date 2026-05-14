function check(actual: any, expected: any) {
  if (actual === expected) {
    console.log("ok");
  } else {
    console.log("bad:" + actual + ":" + expected);
  }
}

check(Number.isFinite(42), true);
check(Number.isNaN(NaN), true);
check(Number.isInteger(42), true);
check(Number.isSafeInteger(2147483647), true);
check(parseInt("ff", 16), 255);
check(parseInt("101", 2), 5);
check(parseInt("z", 36), 35);
check(parseFloat("  -7"), -7);
check(Number.parseInt("ff", 16), 255);
check(Number.parseInt("101", 2), 5);
check(Number.parseInt("z", 36), 35);
check(Number.parseFloat("  -7"), -7);
check((42).toFixed(), "42");
check((42).toFixed(2), "42.00");
check((42).toExponential(), "4.2e+1");
check((42).toExponential(3), "4.200e+1");
check((42).toPrecision(), "42");
check((42).toPrecision(4), "42.00");
