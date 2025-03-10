pub mod upgrade {
    use crate::memory::init_stable_state;
    use crate::types::state::StableState;
    use candid::{CandidType, Principal};
    use serde::{Deserialize, Serialize};
    use shared::types::state::{Controllers, SatelliteId};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    pub struct UpgradeState {
        // Direct stable state: State that is uses stable memory directly as its store. No need for pre/post upgrade hooks.
        #[serde(skip, default = "init_stable_state")]
        pub stable: StableState,

        // Indirect stable state: State that lives on the heap, but is saved into stable memory on upgrades.
        pub heap: UpgradeHeapState,
    }

    #[derive(Default, CandidType, Serialize, Deserialize, Clone)]
    pub struct UpgradeHeapState {
        pub controllers: Controllers,
        pub origins: UpgradeOriginConfigs,
    }

    pub type UpgradeOriginConfigs = HashMap<SatelliteId, UpgradeOriginConfig>;

    #[derive(CandidType, Serialize, Deserialize, Clone)]
    pub struct UpgradeOriginConfig {
        pub key: Principal,
        pub filter: String,
        pub created_at: u64,
        pub updated_at: u64,
    }
}
