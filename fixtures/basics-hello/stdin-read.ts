// Stdin edge case: read piped input and echo it back
const data = await Bun.file("/dev/stdin").text();
console.log(data);
