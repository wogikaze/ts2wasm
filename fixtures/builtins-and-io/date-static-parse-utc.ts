console.log(Date.parse("1970-01-01T00:00:00.000Z"));
console.log(Date.parse("1970-01-01T00:01:02.003Z"));
console.log(Date.UTC(1970));
console.log(Date.UTC(1970, 0));
console.log(Date.UTC(1970, 0, 1, 0, 1, 2, 3));

let fromParse = new Date(Date.parse("1970-01-01T00:00:00.000Z"));
let fromUtc = new Date(Date.UTC(1970, 0, 1));
console.log(fromParse.getTime());
console.log(fromUtc.getTime());
