//! Canonical M0 shared definitions for ts2wasm.

pub mod abi;
pub mod capability;
pub mod test_status;

pub use abi::{AbiFunction, AbiType, RuntimeAbi};
pub use capability::{
    CapabilityManifest, FilesystemCapabilities, NodeHostCapabilities, WasiCapabilities,
};
pub use test_status::{TestRecord, TestStatus};
