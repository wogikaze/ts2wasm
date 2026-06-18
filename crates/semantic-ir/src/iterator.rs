// ---------------------------------------------------------------------------
// Iterator protocol — § 27.1
//
// ECMAScript iterators are protocol-based. The spec defines GetIterator,
// IteratorNext, IteratorStep, IteratorValue, IteratorClose, etc.
// ---------------------------------------------------------------------------

use crate::completion::Completion;
use crate::value::Value;

// ---------------------------------------------------------------------------
// IteratorID — lightweight handle
// ---------------------------------------------------------------------------

/// A unique identifier for an iterator in the runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IteratorID(pub u32);

// ---------------------------------------------------------------------------
// Iterator — § 27.1.2
// ---------------------------------------------------------------------------

/// An Iterator — a wrapper around an iterator object.
///
/// § 27.1.2 — The Iterator Record consists of a `[[Done]]` flag and a
/// `[[NextMethod]]` property.
#[derive(Debug, Clone)]
pub struct Iterator {
    /// The underlying iterator object (the one with `next()`, `return()`, etc.).
    pub object: Value,
    /// The `[[Done]]` flag — whether the iterator has completed.
    pub done: bool,
    /// The `[[NextMethod]]` — the method to call for getting the next value.
    pub next_method: Value,
}

impl Iterator {
    /// Create a new Iterator.
    pub fn new(object: Value, next_method: Value) -> Self {
        Self {
            object,
            done: false,
            next_method,
        }
    }
}

// ---------------------------------------------------------------------------
// GetIterator — § 27.1.1.1
// ---------------------------------------------------------------------------

/// Iterator kind — determines the `return` method to use on iterator close.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IteratorKind {
    /// Synchronous iterator.
    Sync,
    /// Asynchronous iterator.
    Async,
}

/// § 27.1.1.1 — GetIterator(obj, kind).
///
/// Gets an iterator from an object. The spec looks up `Symbol.iterator`
/// on the object, calls it to get the iterator object, then extracts
/// `next` and `return` methods.
///
/// This is a simplified version; the full spec implementation involves:
/// 1. Look up `obj[Symbol.iterator]`
/// 2. Call it to get the iterator object
/// 3. Extract the `next` method
/// 4. Return an Iterator record
pub fn get_iterator(_obj: &Value, _kind: IteratorKind) -> Result<Iterator, Completion> {
    // In the Semantic IR, this will delegate to SpecCall on the iterator method.
    // For now, this is a placeholder that will be filled in by the Spec Kernel (P2).
    // The actual implementation will be a runtime function that calls
    // Symbol.iterator on the object.
    todo!("GetIterator requires runtime object table access")
}

// ---------------------------------------------------------------------------
// IteratorNext — § 27.1.1.2
// ---------------------------------------------------------------------------

/// § 27.1.1.2 — IteratorNext(iteratorRecord, value).
///
/// Calls the `next` method of the iterator. Returns the iterator result object.
///
/// Returns `(done, value)` — the spec actually returns an object `{done, value}`
/// which is then unpacked.
pub fn iterator_next(
    _iterator: &mut Iterator,
    _value: Option<Value>,
) -> Result<IteratorResult, Completion> {
    // In the Spec Kernel (P2), this will:
    // 1. Call iterator.next_method with the given value
    // 2. Validate the result is an Object
    // 3. Return the IteratorResult
    todo!("IteratorNext requires runtime call machinery")
}

/// The result of IteratorNext — `{done, value}`.
#[derive(Debug, Clone)]
pub struct IteratorResult {
    pub done: bool,
    pub value: Value,
}

// ---------------------------------------------------------------------------
// IteratorStep — § 27.1.1.3
// ---------------------------------------------------------------------------

/// § 27.1.1.3 — IteratorStep(iteratorRecord).
///
/// Calls IteratorNext and returns the result if not done.
/// Returns `None` if the iterator is done (the `done: true` case).
pub fn iterator_step(
    iterator: &mut Iterator,
) -> Result<Option<IteratorResult>, Completion> {
    let result = iterator_next(iterator, None)?;
    if result.done {
        iterator.done = true;
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

// ---------------------------------------------------------------------------
// IteratorValue — § 27.1.1.4
// ---------------------------------------------------------------------------

/// § 27.1.1.4 — IteratorValue(iteratorResult).
///
/// Extracts the value from an iterator result.
pub fn iterator_value(result: &IteratorResult) -> Value {
    result.value.clone()
}

// ---------------------------------------------------------------------------
// IteratorClose — § 27.1.1.6
// ---------------------------------------------------------------------------

/// § 27.1.1.6 — IteratorClose(iteratorRecord, completion).
///
/// Calls the `return` method of the iterator (if it exists) and returns
/// the original completion.
pub fn iterator_close(
    _iterator: Iterator,
    _completion: Completion,
    _kind: IteratorKind,
) -> Completion {
    // In the Spec Kernel (P2), this will:
    // 1. Get iterator.return if it exists
    // 2. If it doesn't exist, return the original completion
    // 3. Call it
    // 4. If the call throws, return the throw completion
    // 5. Otherwise, return the original completion
    todo!("IteratorClose requires runtime call machinery")
}

// ---------------------------------------------------------------------------
// IteratorComplete — § 27.1.1.5
// ---------------------------------------------------------------------------

/// § 27.1.1.5 — IteratorComplete(iteratorResult).
///
/// Validates that the iterator result is an object and extracts the value.
pub fn iterator_complete(_result: Value) -> Result<Value, Completion> {
    // In the Spec Kernel, this will validate the result is an object
    // and extract the `value` property.
    todo!("IteratorComplete requires runtime object access")
}

// ---------------------------------------------------------------------------
// CreateIterResultObject — § 27.1.1.7
// ---------------------------------------------------------------------------

/// § 27.1.1.7 — CreateIterResultObject(value, done).
///
/// Creates the `{value, done}` object returned by `next()`.
pub fn create_iter_result_object(_value: Value, _done: bool) -> Value {
    // In the Spec Kernel, this creates an Object with `value` and `done` properties.
    todo!("CreateIterResultObject requires runtime object creation")
}
