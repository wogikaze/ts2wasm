let target = { value: 0 };
function getTarget() {
  return target;
}
function key() {
  return "value";
}
getTarget()[key()] &&= 1;
