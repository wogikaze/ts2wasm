class Box {
  constructor() {
    console.log(eval("new.target === Box"));
  }
}

new Box();
