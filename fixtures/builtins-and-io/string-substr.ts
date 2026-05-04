// String.prototype.substr (Annex B legacy method)
console.log("'" + 'abc'.substr(0, false) + "'");
console.log("'" + 'abc'.substr(0, NaN) + "'");
console.log("'" + 'abc'.substr(0, '') + "'");
console.log("'" + 'abc'.substr(0, null) + "'");
console.log("'" + 'abc'.substr(0, -1) + "'");
console.log("'" + 'abc'.substr(1, -1) + "'");
console.log("'" + 'abc'.substr(2, -1) + "'");
console.log("'" + 'abc'.substr(3, -1) + "'");
console.log("'" + 'abc'.substr(1) + "'");
console.log("'" + 'abc'.substr(1, 1) + "'");
console.log("'" + 'abc'.substr(0, 3) + "'");
console.log("'" + 'abc'.substr(0, 100) + "'");
console.log("'" + 'abc'.substr(-1, 1) + "'");
console.log("'" + 'abc'.substr(-2, 2) + "'");
console.log("'" + 'abc'.substr(-5) + "'");
