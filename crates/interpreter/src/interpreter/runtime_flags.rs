use primitives::hardfork::SpecId;
use alloy_rpc_types_trace::geth::SentioDebugTracingOptions;

use super::RuntimeFlag;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RuntimeFlags {
    pub is_static: bool,
    pub is_eof_init: bool,
    pub is_eof: bool,
    pub spec_id: SpecId,
    pub sentio_config: SentioDebugTracingOptions,
}

impl RuntimeFlag for RuntimeFlags {
    fn is_static(&self) -> bool {
        self.is_static
    }

    fn is_eof(&self) -> bool {
        self.is_eof
    }

    fn is_eof_init(&self) -> bool {
        self.is_eof_init
    }

    fn spec_id(&self) -> SpecId {
        self.spec_id
    }
    
    fn sentio_config(&self) -> &SentioDebugTracingOptions {
        &self.sentio_config
    }
}
