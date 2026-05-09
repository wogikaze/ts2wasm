// Intl basic usage — should produce unsupported diagnostic
const formatter = new Intl.DateTimeFormat("en-US");
console.log(formatter.format(new Date()));
