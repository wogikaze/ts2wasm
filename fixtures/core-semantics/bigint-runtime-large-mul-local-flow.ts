let branch = 1n;
let cond = true;
if (cond) {
  branch = 18446744073709551616n;
} else {
  branch = 3n;
}
console.log(branch * 3n);

let looped = 2n;
let i = 0;
while (i < 1) {
  looped = 99999999999999999999n;
  i = i + 1;
}
console.log(looped * branch);

let switched = 4n;
switch (1) {
  case 0:
    switched = 5n;
    break;
  case 1:
    switched = 12345678901234567890n;
    break;
  default:
    switched = 6n;
}
console.log(switched * 7n);

let tried = 8n;
try {
  tried = -123456789012345678901n;
} catch (e) {
  tried = 9n;
} finally {
  tried = tried * 1n;
}
console.log(tried * -11n);
