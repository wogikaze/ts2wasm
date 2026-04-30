type Id = number;

type Box<T> = {
  value: T;
};

type MaybePair<T extends string | number, U = T> =
  Box<T> | ({ left: T; right: U } & { tag?: "pair" });

export type Point = {
  x: number;
  y: number;
  meta: { created: number };
  translate: (dx: number, dy: number) => number;
};

function sum(point: Point): Id {
  return point.x + point.y;
}

function readBox(box: Box<number>): number {
  return box.value;
}

let origin: Point = {
  x: 2,
  y: 3,
  meta: { created: 1 }
};

console.log(sum(origin));
console.log(readBox({ value: 4 }));
