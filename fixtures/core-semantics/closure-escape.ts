function toText(v) {
  return v;
}

var out = "";
var i = 0;
while (i < 2000) {
  out = out + toText(i);
  i = i + 1;
}

console.log(out.length);
