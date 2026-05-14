// Set algebraic methods: isDisjointFrom, isSubsetOf, isSupersetOf,
// union, intersection, difference, symmetricDifference

let s1 = new Set<number>();
s1.add(1);
s1.add(2);
s1.add(3);

let s2 = new Set<number>();
s2.add(3);
s2.add(4);
s2.add(5);

let s3 = new Set<number>();
s3.add(4);
s3.add(5);
s3.add(6);

// isDisjointFrom
console.log(s1.isDisjointFrom(s2));
console.log(s1.isDisjointFrom(s3));
console.log(s1.isDisjointFrom(new Set()));

// isSubsetOf
console.log(s1.isSubsetOf(s2));
console.log(s1.isSubsetOf(s1));
console.log(new Set().isSubsetOf(s1));

// isSupersetOf
console.log(s1.isSupersetOf(s2));
console.log(s1.isSupersetOf(s1));
console.log(s1.isSupersetOf(new Set()));

// union, intersection, difference, symmetricDifference
// (return Set — compile-time validation in build_smoke test)
s1.union(s2);
s1.intersection(s2);
s1.difference(s2);
s1.symmetricDifference(s2);
