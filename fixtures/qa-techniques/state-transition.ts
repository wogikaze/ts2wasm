class Reservation {
    state: string;

    constructor() {
        this.state = "Draft";
    }

    confirm(): boolean {
        if (this.state === "Draft") {
            this.state = "Confirmed";
            return true;
        }
        return false;
    }

    complete(): boolean {
        if (this.state === "Confirmed") {
            this.state = "Completed";
            return true;
        }
        return false;
    }

    cancel(): boolean {
        if (this.state === "Draft" || this.state === "Confirmed") {
            this.state = "Cancelled";
            return true;
        }
        return false;
    }
}

let res1 = new Reservation();
console.log(res1.state);
console.log(res1.confirm());
console.log(res1.state);
console.log(res1.complete());
console.log(res1.state);
console.log(res1.cancel());

let res2 = new Reservation();
console.log(res2.cancel());
console.log(res2.confirm());
