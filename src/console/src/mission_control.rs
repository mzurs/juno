use crate::controllers::update_mission_control_controllers;
use crate::store::{
    add_mission_control, delete_mission_control, get_mission_control, init_empty_mission_control,
    redeem_invitation_code,
};
use crate::types::state::MissionControl;
use crate::wasm::mission_control_wasm_arg;
use candid::Principal;
use ic_cdk::{notify, print};
use shared::constants::CREATE_MISSION_CONTROL_CYCLES;
use shared::env::OBSERVATORY;
use shared::ic::create_canister_install_code;
use shared::types::interface::{MissionControlId, ObservatoryAddMissionControlArgs, UserId};

pub async fn init_user_mission_control(
    console: &Principal,
    caller: &Principal,
    invitation_code: &Option<String>,
) -> Result<MissionControl, String> {
    let result = get_mission_control(caller);
    match result {
        Err(error) => Err(error.to_string()),
        Ok(mission_control) => match mission_control {
            Some(mission_control) => Ok(mission_control),
            None => match invitation_code {
                None => Err("No invitation code provided.".to_string()),
                Some(invitation_code) => {
                    redeem_invitation_code(caller, invitation_code)?;

                    create_mission_control(caller, console).await
                }
            },
        },
    }
}

async fn create_mission_control(
    user: &UserId,
    console: &Principal,
) -> Result<MissionControl, String> {
    init_empty_mission_control(user);

    let wasm_arg = mission_control_wasm_arg(user);

    let create = create_canister_install_code(
        Vec::from([*console, *user]),
        &wasm_arg,
        CREATE_MISSION_CONTROL_CYCLES,
    )
    .await;

    match create {
        Err(e) => {
            // We delete the pending empty mission control center from the list - e.g. this can happens if manager is out of cycles and user would be blocked
            delete_mission_control(user);
            Err(["Canister cannot be initialized.", &e].join(""))
        }
        Ok(mission_control_id) => {
            let mission_control = add_mission_control(user, &mission_control_id);

            notify_observatory(&mission_control_id, user);

            update_mission_control_controllers(&mission_control_id, user).await?;

            Ok(mission_control)
        }
    }
}

fn notify_observatory(mission_control_id: &MissionControlId, owner: &UserId) {
    let observatory = Principal::from_text(OBSERVATORY).unwrap();

    let args: ObservatoryAddMissionControlArgs = ObservatoryAddMissionControlArgs {
        mission_control_id: *mission_control_id,
        owner: *owner,
    };

    // We silence the error which is not optimal but adding the newly created mission control center to the observatory is not functionally speaking a must have
    let () = notify(observatory, "add_mission_control", (args,)).unwrap_or_else(|_| {
        print(format!(
            "Mission control center ({}) cannot be added to the observatory.",
            mission_control_id
        ))
    });
}