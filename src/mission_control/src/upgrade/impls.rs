use crate::types::state::StableState;
use crate::upgrade::types::upgrade::UpgradeStableState;
use shared::types::state::ControllerScope;
use shared::upgrade::upgrade_controllers;

impl From<&UpgradeStableState> for StableState {
    fn from(state: &UpgradeStableState) -> Self {
        StableState {
            controllers: upgrade_controllers(state.controllers.clone(), ControllerScope::Admin),
            user: state.user.clone(),
            satellites: state.satellites.clone(),
            archive: state.archive.clone(),
        }
    }
}
