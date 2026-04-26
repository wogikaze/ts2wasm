// Test default parameters
function greet(name = "World") {
    console.log("Hello, " + name);
}
greet(); // Should print "Hello, World"
greet("Alice"); // Should print "Hello, Alice"
console.log(0); // Exit marker
