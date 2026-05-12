// basic function return
function add(a: number, b: number): number {
  return a + b;
}
console.log("add " + add(2, 3));

// early return
function find(arr: number[], target: number): number {
  for (let i = 0; i < 3; i = i + 1) {
    if (arr[i] === target) {
      return i;
    }
  }
  return -1;
}
let nums = [10, 20, 30];
console.log("find " + find(nums, 20));
console.log("find missing " + find(nums, 99));

// conditional return
function isPositive(n: number): boolean {
  if (n > 0) {
    return true;
  }
  return false;
}
console.log("isPositive " + isPositive(5));
console.log("isPositive " + isPositive(-3));

// return from nested blocks
function testNested(x: number): number {
  if (x > 0) {
    if (x > 10) {
      return 100;
    }
    return 10;
  }
  return 0;
}
console.log("nested " + testNested(5));
console.log("nested " + testNested(15));
console.log("nested " + testNested(-1));

// void return
function logMessage(msg: string): void {
  console.log("msg: " + msg);
  return;
}
logMessage("test");
