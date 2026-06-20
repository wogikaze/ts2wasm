pub type FrameId = u32;

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub return_address: u32,
    pub previous_frame: Option<FrameId>,
    pub stack_base: u32,
    pub local_count: u32,
    pub function: Option<FunctionRef>,
    pub env: crate::env::EnvRef,
}

#[derive(Debug, Clone)]
pub struct FunctionRef(pub u32);
