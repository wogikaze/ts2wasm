// Characterization test fixture: array lowering domain coverage
// Exercise array literal, spread, destructuring, callback methods
// Used by P9 lower_expr domain split acceptance

// Array literal lowering
const arr1: number[] = [1, 2, 3];
const arr2: string[] = ["a", "b", "c"];

// Array spread in literals
const arr3: number[] = [...arr1, 4, 5];
const arr4: string[] = ["x", ...arr2, "y"];

// Array index access and assignment
const first: number = arr1[0];
arr1[1] = 42;

// Array length
const len: number = arr1.length;

// Array concat
const merged: number[] = arr1.concat(arr3);

// Array slice
const sliced: number[] = arr1.slice(1, 3);

// Array indexOf
const idx: number = arr1.indexOf(42);

// Array push and pop
arr1.push(99);
const popped: number = arr1.pop();

// Array join
const joined: string = arr2.join(",");

// Array includes
const hasTwo: boolean = arr1.includes(2);

// Array reverse
arr1.reverse();

// Nested arrays
const nested: number[][] = [[1, 2], [3, 4]];
const inner: number[] = nested[0];
