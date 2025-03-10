use crate::STATE;
use ic_cdk::caller;
use shared::controllers::is_admin_controller;
use shared::types::state::Controllers;

pub fn caller_is_admin_controller() -> Result<(), String> {
    let caller = caller();
    let controllers: Controllers = STATE.with(|state| state.borrow().heap.controllers.clone());

    if is_admin_controller(caller, &controllers) {
        Ok(())
    } else {
        Err("Caller is not a controller of the satellite.".to_string())
    }
}
