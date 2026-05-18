let body = "}";

try {
  Function(body);
} catch (err) {
  console.log(err.name);
}
