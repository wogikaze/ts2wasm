class BoundBox {
  value;
  extra;
  isSelf;

  constructor(value, extra) {
    this.value = value;
    this.extra = extra;
    this.isSelf = new.target === BoundBox;
  }
}

const BoundWithValue = BoundBox.bind(null, 7);
const first = new BoundWithValue(3);
console.log(first.value);
console.log(first.extra);
console.log(first.isSelf);

const second = new BoundWithValue(11);
console.log(second.value);
console.log(second.extra);
console.log(second.isSelf);
