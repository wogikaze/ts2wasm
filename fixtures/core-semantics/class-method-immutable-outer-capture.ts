let suffix = "-capture";

class Reader {
  read(prefix) {
    return prefix + suffix;
  }
}

let reader = new Reader();
console.log(reader.read("class"));
