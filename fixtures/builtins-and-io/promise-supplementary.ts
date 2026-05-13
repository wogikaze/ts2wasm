const resolvers = Promise.withResolvers();
console.log(Object.keys(resolvers).join(","));

const aggregate = new AggregateError(["first", "second"], "combined failure");
console.log(aggregate.name);
console.log(aggregate.message);
console.log(aggregate.errors.length);

Promise.any([Promise.reject("miss"), Promise.resolve(7)]);
console.log("any-done");

Promise.allSettled([Promise.resolve(1), Promise.reject(2)]);
console.log("allSettled-done");
