const sliced: any = Iterator.from("abcd").drop(1).take(2).toArray();
console.log(sliced.length);

const exhausted: any = Iterator.from("xy").drop(5).toArray();
console.log(exhausted.length);
