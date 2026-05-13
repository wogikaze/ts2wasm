let text = "hello";
let index = 0;
let first = text[index];
let len = text.length;
let sum = 1 + 2;
let strict = sum === 3;
let loose = sum == "3";
let less = index < len;
let lessEqual = index <= len;
let greater = len > index;
let greaterEqual = len >= index;
let notFalse = !false;
let logger = console.log;

if (strict) {
  console.log(first);
} else {
  console.log(len);
}

while (index < 1) {
  index = index + 1;
}

text.toString();
console.log(loose);
console.log(less);
console.log(lessEqual);
console.log(greater);
console.log(greaterEqual);
console.log(notFalse);
