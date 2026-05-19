function run() {
  let result = eval('for (var { drop, ...rest } of [{ drop: 1, keep: "ok", next: 2 }]) {} rest.keep + ":" + rest.next + ":" + drop');
  console.log(result);
  console.log(rest.keep);
  console.log(rest.next);
  console.log(drop);
}

run();
