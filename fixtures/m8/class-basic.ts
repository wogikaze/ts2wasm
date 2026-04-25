class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }

  sum() {
    return this.x + this.y;
  }
}

let p = new Point(1, 2);
console.log(p.sum());
