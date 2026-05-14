let a = new Set();
a.add(1);
a.add(2);
a.add(3);

let b = new Set();
b.add(2);
b.add(3);
b.add(4);

let empty = new Set();

// isDisjointFrom
console.log(a.isDisjointFrom(b));   // false (share 2, 3)
console.log(a.isDisjointFrom(empty)); // true

// isSubsetOf
console.log(a.isSubsetOf(b));   // false (1 not in b)
console.log(empty.isSubsetOf(a)); // true
let c = new Set();
c.add(2);
c.add(3);
console.log(c.isSubsetOf(a));   // true (2,3 are in a)

// isSupersetOf
console.log(a.isSupersetOf(c));   // true (a contains 2,3)
console.log(a.isSupersetOf(b));   // false (4 not in a)
console.log(a.isSupersetOf(empty)); // true

// union — check results via iteration on original sets
let u = a.union(b);
console.log("union_done");

// intersection
let i = a.intersection(b);
console.log("intersection_done");

// difference
let d = a.difference(b);
console.log("difference_done");

// symmetricDifference
let sd = a.symmetricDifference(b);
console.log("symdiff_done");
