function callback(value, index, array) {
  return value + index + array.length;
}

const result = Array.prototype.map.call([1, 2], callback);
console.log(result.length);
