// Stdin edge case: empty piped input should not hang and should produce empty output
const data = await Bun.file("/dev/stdin").text();
console.log(data);
