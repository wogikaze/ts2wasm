// Diagnostic fixture for unsupported async iteration tracked by issue 230.
for await (var value of values) {
  console.log(value);
}
