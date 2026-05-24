function formatNumber(value: any) {
  console.log(value.toFixed(2));
  console.log(value.toExponential(1));
  console.log(value.toPrecision(4));
}

formatNumber(42);
