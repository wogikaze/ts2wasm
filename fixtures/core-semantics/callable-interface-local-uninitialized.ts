interface I<T> {
    <U extends T>(u: U): U;
}
var i: I<string>;
var y = i(""); // should report issue-5195: callable interface-typed local
console.log(y);
