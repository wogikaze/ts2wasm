import { Decimal } from "./decimal-lib";

console.log("=== decimal.js tests ===");
const a = new Decimal("10.5");
const b = new Decimal("3.2");
console.log("a =", a.toString());
console.log("b =", b.toString());
console.log("a + b =", a.plus(b).toString());
console.log("a - b =", a.minus(b).toString());
console.log("a * b =", a.times(b).toString());
console.log("a / b =", a.dividedBy(b, 6).toString());

const c = new Decimal("-7.25");
console.log("c =", c.toString());
console.log("abs(c) =", c.absoluteValue().toString());
console.log("neg(c) =", c.negated().toString());
console.log("c + a =", c.plus(a).toString());

const d = new Decimal("0.1");
const e = new Decimal("0.2");
console.log("0.1 + 0.2 =", d.plus(e).toString());

const big = new Decimal("12345678901234567890");
const big2 = new Decimal("98765432109876543210");
console.log("big + big2 =", big.plus(big2).toString());
console.log("big * 2 =", big.times(2).toString());
console.log("big / 2 =", big.dividedBy(2).toString());

const f = new Decimal("100");
console.log("f toFixed(2) =", f.toFixed(2));
console.log("f toFixed(5) =", f.toFixed(5));

console.log("isZero 0:", new Decimal(0).isZero());
console.log("isPositive 5:", new Decimal(5).isPositive());
console.log("isNegative -3:", new Decimal(-3).isNegative());
console.log("round trip 3.14:", new Decimal(3.14).toString());
