// Braceless for-in loop bodies (issue I-20260515-GN2SJC)
// Currently works after commit 73d789796 frontend: parse braceless for loop bodies
var obj = { a: 1, b: 2, c: 3 };
var result = "";
for (var key in obj)
  result += key;
console.log(result === "abc" ? "pass" : "fail: " + result);
