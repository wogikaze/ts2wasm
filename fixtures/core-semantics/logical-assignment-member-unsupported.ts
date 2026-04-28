let target = { value: 0 };
function getTarget() {
  return target;
}
getTarget().value &&= 1;
