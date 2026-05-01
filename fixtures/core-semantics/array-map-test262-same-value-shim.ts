// @ts-nocheck
var assert = {
  sameValue: function (actual, expected, message) {
    if (actual === expected) {
      return;
    }
    console.log("__TS2WASM_TEST262_ASSERT_FAIL__");
  }
};

var callCnt = 0;

function callbackfn(val, idx, obj) {
  callCnt++;
  return 1;
}

var srcArr = new Array(10);
srcArr[1] = undefined;
var resArr = srcArr.map(callbackfn);

assert.sameValue(resArr.length, 10, "resArr.length");
assert.sameValue(callCnt, 1, "callCnt");
console.log("ok");
