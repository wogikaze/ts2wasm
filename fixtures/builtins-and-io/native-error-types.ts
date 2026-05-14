try { throw new EvalError("eval error"); } catch (e) { console.log(e.name); }
try { throw new RangeError("range error"); } catch (e) { console.log(e.name); }
try { throw new ReferenceError("ref error"); } catch (e) { console.log(e.name); }
try { throw new SyntaxError("syntax error"); } catch (e) { console.log(e.name); }
try { throw new TypeError("type error"); } catch (e) { console.log(e.name); }
try { throw new URIError("uri error"); } catch (e) { console.log(e.name); }
try { throw new Error("plain error"); } catch (e) { console.log(e.name); }
