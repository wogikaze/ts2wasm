// Minimal decimal.js subset - arbitrary precision decimal arithmetic
// Uses string-based representation for precision

export class Decimal {
  private val: string;
  private isNeg: boolean;

  constructor(v: Decimal | string | number) {
    if (v instanceof Decimal) {
      this.val = v.val;
      this.isNeg = v.isNeg;
      return;
    }
    if (typeof v === "number") {
      const s = String(v);
      this.isNeg = s[0] === "-";
      this.val = this.isNeg ? s.slice(1) : s;
      return;
    }
    const s = v.trim();
    this.isNeg = s[0] === "-";
    this.val = this.isNeg ? s.slice(1) : s;
  }

  private static normalize(a: string): string {
    let dot = a.indexOf(".");
    if (dot === -1) { a = a + ".0"; dot = a.indexOf("."); }
    // Remove trailing zeros after decimal
    let end = a.length - 1;
    while (end > dot && a[end] === "0") end--;
    a = a.slice(0, end + 1);
    if (a[a.length - 1] === ".") a = a + "0";
    return a;
  }

  private static align(a: string, b: string): [string, string, number] {
    a = Decimal.normalize(a);
    b = Decimal.normalize(b);
    let dotA = a.indexOf(".");
    let dotB = b.indexOf(".");
    let fracA = a.length - dotA - 1;
    let fracB = b.length - dotB - 1;
    let frac = fracA > fracB ? fracA : fracB;
    while (a.length - dotA - 1 < frac) a = a + "0";
    while (b.length - dotB - 1 < frac) b = b + "0";
    a = a.slice(0, dotA) + a.slice(dotA + 1);
    b = b.slice(0, dotB) + b.slice(dotB + 1);
    // Pad with leading zeros
    while (a.length < b.length) a = "0" + a;
    while (b.length < a.length) b = "0" + b;
    return [a, b, frac];
  }

  private static absCompare(a: string, b: string): number {
    [a, b] = Decimal.align(a, b);
    if (a.length !== b.length) return a.length > b.length ? 1 : -1;
    for (let i = 0; i < a.length; i++) {
      if (a[i] > b[i]) return 1;
      if (a[i] < b[i]) return -1;
    }
    return 0;
  }

  private static addStr(a: string, b: string): string {
    let carry = 0;
    let result = "";
    let i = a.length - 1;
    let j = b.length - 1;
    while (i >= 0 || j >= 0 || carry > 0) {
      const da = i >= 0 ? parseInt(a[i], 10) : 0;
      const db = j >= 0 ? parseInt(b[j], 10) : 0;
      const sum = da + db + carry;
      result = String(sum % 10) + result;
      carry = sum >= 10 ? 1 : 0;
      i--;
      j--;
    }
    return result;
  }

  private static subStr(a: string, b: string): string {
    let borrow = 0;
    let result = "";
    let i = a.length - 1;
    let j = b.length - 1;
    while (i >= 0) {
      const da = parseInt(a[i], 10) - borrow;
      const db = j >= 0 ? parseInt(b[j], 10) : 0;
      if (da < db) {
        result = String(da + 10 - db) + result;
        borrow = 1;
      } else {
        result = String(da - db) + result;
        borrow = 0;
      }
      i--;
      j--;
    }
    // Remove leading zeros
    let k = 0;
    while (k < result.length - 1 && result[k] === "0") k++;
    return result.slice(k);
  }

  private static mulStr(a: string, b: string): string {
    const result = new Array(a.length + b.length).fill(0);
    for (let i = a.length - 1; i >= 0; i--) {
      for (let j = b.length - 1; j >= 0; j--) {
        const prod = parseInt(a[i], 10) * parseInt(b[j], 10) + result[i + j + 1];
        result[i + j + 1] = prod % 10;
        result[i + j] += Math.floor(prod / 10);
      }
    }
    let s = result.join("");
    let k = 0;
    while (k < s.length - 1 && s[k] === "0") k++;
    return s.slice(k);
  }

  private static divStr(a: string, b: string, precision: number): string {
    let result = "";
    let remainder = "0";
    let dot = false;
    let digits = 0;
    let ai = 0;
    let aDone = false;

    while (digits < precision + 1) {
      // Bring down next digit
      let current = remainder === "0" ? "" : remainder;
      if (!aDone && ai < a.length) {
        current = current + a[ai];
        ai++;
      } else if (!aDone && ai >= a.length && !dot) {
        dot = true;
        if (result === "") result = "0";
        result = result + ".";
        current = current + "0";
        aDone = true;
      } else if (aDone || ai >= a.length) {
        if (!dot) {
          dot = true;
          if (result === "") result = "0";
          result = result + ".";
        }
        current = current + "0";
      }

      if (current === "" || current === "0") {
        remainder = "0";
        if (!dot) {
          result = result + "0";
        } else {
          result = result + "0";
          digits++;
        }
      } else {
        let count = 0;
        while (Decimal.absCompare(current, b) >= 0) {
          current = Decimal.subStr(current, b);
          count++;
        }
        remainder = current;
        result = result + String(count);
        if (dot) digits++;
      }
      if (ai >= a.length && remainder === "0" && dot) break;
    }
    return result;
  }

  plus(other: Decimal | string | number): Decimal {
    const o = other instanceof Decimal ? other : new Decimal(other);
    if (this.isNeg === o.isNeg) {
      const [a, b] = Decimal.align(this.val, o.val);
      const sum = Decimal.addStr(a, b);
      const result = new Decimal("0");
      result.val = sum;
      result.isNeg = this.isNeg;
      return result;
    }
    const cmp = Decimal.absCompare(this.val, o.val);
    if (cmp === 0) return new Decimal(0);
    const bigger = cmp > 0 ? this : o;
    const smaller = cmp > 0 ? o : this;
    const [a, b] = Decimal.align(bigger.val, smaller.val);
    const diff = Decimal.subStr(a, b);
    const result = new Decimal("0");
    result.val = diff;
    result.isNeg = bigger.isNeg;
    return result;
  }

  minus(other: Decimal | string | number): Decimal {
    const o = other instanceof Decimal ? other : new Decimal(other);
    return this.plus(new Decimal(o.isNeg ? o.val.slice(1) : "-" + o.val));
  }

  times(other: Decimal | string | number): Decimal {
    const o = other instanceof Decimal ? other : new Decimal(other);
    const [a, b, frac] = Decimal.align(this.val, o.val);
    const prod = Decimal.mulStr(a, b);
    const totalFrac = frac * 2;
    let intPart: string;
    let fracPart: string;
    if (totalFrac >= prod.length) {
      intPart = "0";
      fracPart = "0".repeat(totalFrac - prod.length) + prod;
    } else {
      intPart = prod.slice(0, prod.length - totalFrac) || "0";
      fracPart = prod.slice(prod.length - totalFrac);
    }
    const result = new Decimal("0");
    result.val = intPart + "." + fracPart;
    result.isNeg = this.isNeg !== o.isNeg;
    result.val = Decimal.normalize(result.val);
    return result;
  }

  dividedBy(other: Decimal | string | number, precision?: number): Decimal {
    const o = other instanceof Decimal ? other : new Decimal(other);
    const p = precision || 20;
    const a = this.val.indexOf(".") >= 0 ? this.val : this.val + ".0";
    const b = o.val.indexOf(".") >= 0 ? o.val : o.val + ".0";
    // Scale both to integers
    const dotA = a.indexOf(".");
    const dotB = b.indexOf(".");
    const scale = a.length - dotA - 1 > b.length - dotB - 1
      ? a.length - dotA - 1 : b.length - dotB - 1;
    let intA = a.slice(0, dotA) + a.slice(dotA + 1);
    let intB = b.slice(0, dotB) + b.slice(dotB + 1);
    while (intA.length - (a.length - dotA - 1) < scale) intA = intA + "0";
    while (intB.length - (b.length - dotB - 1) < scale) intB = intB + "0";

    const div = Decimal.divStr(intA, intB, p);
    const result = new Decimal("0");
    result.val = div;
    result.isNeg = this.isNeg !== o.isNeg;
    result.val = Decimal.normalize(result.val);
    return result;
  }

  toString(): string {
    let s = Decimal.normalize(this.val);
    if (this.isNeg && s !== "0" && s !== "0.0") s = "-" + s;
    return s;
  }

  valueOf(): number {
    return parseFloat(this.toString());
  }

  toFixed(dp: number): string {
    let s = Decimal.normalize(this.val);
    let dot = s.indexOf(".");
    let frac = s.length - dot - 1;
    if (frac > dp) {
      // Truncate
      s = s.slice(0, s.length - (frac - dp));
    } else if (frac < dp) {
      while (s.length - dot - 1 < dp) s = s + "0";
    }
    if (this.isNeg && s !== "0" && s !== "0.0") s = "-" + s;
    return s;
  }

  isZero(): boolean {
    const s = Decimal.normalize(this.val);
    return s === "0" || s === "0.0";
  }

  isPositive(): boolean {
    return !this.isNeg && !this.isZero();
  }

  isNegative(): boolean {
    return this.isNeg && !this.isZero();
  }

  absoluteValue(): Decimal {
    const r = new Decimal(this);
    r.isNeg = false;
    return r;
  }

  negated(): Decimal {
    const r = new Decimal(this);
    if (!this.isZero()) r.isNeg = !r.isNeg;
    return r;
  }
}
