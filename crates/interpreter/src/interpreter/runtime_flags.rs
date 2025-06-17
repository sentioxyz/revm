use primitives::hardfork::SpecId;
use alloy_rpc_types_trace::geth::SentioDebugTracingOptions;

use super::RuntimeFlag;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Runtime flags that control interpreter execution behavior.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RuntimeFlags {
    /// Whether the current execution context is static (read-only).
    pub is_static: bool,
    /// The current EVM specification ID.
    pub spec_id: SpecId,
    /// Sentio config
    pub sentio_config: SentioDebugTracingOptions,
}

impl RuntimeFlag for RuntimeFlags {
    fn is_static(&self) -> bool {
        self.is_static
    }

    fn spec_id(&self) -> SpecId {
        self.spec_id
    }
    
    fn sentio_config(&self) -> &SentioDebugTracingOptions {
        &self.sentio_config
    }
}
