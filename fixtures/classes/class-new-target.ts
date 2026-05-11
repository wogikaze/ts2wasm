class NewTargetBox {
    constructor() {
        this.isSelf = new.target === NewTargetBox;
    }
}

let value = new NewTargetBox();
console.log(value.isSelf);
