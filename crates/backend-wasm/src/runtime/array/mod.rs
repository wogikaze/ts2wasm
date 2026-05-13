mod accessor;
pub mod catalog;
pub mod emit;
mod iteration;
mod iterator;
mod mutator;

// Sparse array holes are represented by unset presence-bitmap bits in the
// shared array payload; runtime helpers must preserve or intentionally
// materialize those holes according to the operation's ECMAScript semantics.
