// Nested objects
// Exercises: object within object, chained property access

let nested = { a: { b: 1 } };
console.log(nested.a.b);

let config = {
  server: { host: "localhost", port: 8080 },
  db: { name: "test", user: "admin" }
};
console.log(config.server.host);
console.log(config.server.port);
console.log(config.db.name);

// Mutate nested property
config.server.port = 9090;
console.log(config.server.port);

// Access deep missing
console.log(config.db.password);

// Variable chain
let outer = "server";
let inner = "host";
console.log(config[outer][inner]);
