//! Canonical M0 shared definitions for ts2wasm.

pub mod abi;
pub mod ast;
pub mod capability;
pub mod diagnostic;
pub mod test_helpers;
pub mod test_status;

pub use abi::{AbiFunction, AbiType, RuntimeAbi};
pub use capability::{
    CapabilityManifest, ClockCapabilities, FilesystemCapabilities, NodeHostCapabilities,
    WasiCapabilities,
};
pub use diagnostic::Span;
pub use test_status::{TestRecord, TestStatus, TrackingId};
