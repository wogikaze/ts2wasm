// Basic this binding test
// Note: Proper this binding requires method call implementation (issue 016)
// This fixture tests that 'this' parses and compiles without errors

function testThis() {
    const x = this;
    return x;
}

testThis();
