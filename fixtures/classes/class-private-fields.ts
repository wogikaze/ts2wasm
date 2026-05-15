// Private fields and methods across inheritance
class Base {
  #baseSecret = "base-secret";
  getBaseSecret() {
    return this.#baseSecret;
  }
}

class Derived extends Base {
  #derivedSecret = "derived-secret";
  getDerivedSecret() {
    return this.#derivedSecret;
  }
  getBoth() {
    return this.getBaseSecret() + ":" + this.#derivedSecret;
  }
}

const d = new Derived();
console.log(d.getBaseSecret());
console.log(d.getDerivedSecret());
console.log(d.getBoth());
