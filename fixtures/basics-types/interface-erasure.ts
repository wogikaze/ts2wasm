interface Point {
  x: number;
  y: number;
}

export interface NamedPoint {
  name: string;
  point: Point;
}

function sum(point: Point): number {
  return point.x + point.y;
}

let origin: Point = { x: 2, y: 3 };
console.log(sum(origin));
