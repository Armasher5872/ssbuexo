use {
    crate::functions::{
        FIGHTER_BOOL_1,
        FIGHTER_BOOL_2,
        STATUS_KIND,
        get_player_number
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::{
            L2CValueType,
            lua_const::*,
        },
        lua2cpp::{
            L2CFighterBase,
            L2CFighterCommon
        },
    },
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_GAMEWATCH )]
pub fn gamewatch_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = smash::app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let mut globals = fighter.globals_mut().clone();
		let bomb_check = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
		if fighter_kind == FIGHTER_KIND_GAMEWATCH {
			if let L2CValueType::Void = globals["gamewatch_frame"].val_type {
				globals["gamewatch_frame"] = 0.0.into();
			}        
			if globals["gamewatch_frame"].get_num() < 60.0 {
				globals["gamewatch_frame"] = (globals["gamewatch_frame"].get_num() + 1.0).into();
			}
			if motion_kind == hash40("attack_air_f") {
				if MotionModule::frame(module_accessor) == 0.0 {
					globals["gamewatch_frame"] = 0.0.into();
				}
			}
			if globals["gamewatch_frame"].get_num() > 13.0 && globals["gamewatch_frame"].get_num() <= 44.0 {
				*bomb_check = true;
			}
			else {
				*bomb_check = false;
			}
		}
	}
}

#[weapon_frame( agent = WEAPON_KIND_GAMEWATCH_BOMB )]
fn gamewatch_bomb_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let weapon_kind = smash::app::utility::get_kind(module_accessor) as i32;
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let bomb_check = &mut FIGHTER_BOOL_1[get_player_number(owner_module_accessor)];
		let bomb_explosion = &mut FIGHTER_BOOL_2[get_player_number(owner_module_accessor)];
		if weapon_kind == WEAPON_KIND_GAMEWATCH_BOMB {
			if WorkModule::is_flag(module_accessor, *WEAPON_GAMEWATCH_BOMB_STATUS_WORK_FLAG_DAMAGE) {
				if *bomb_check {
					*bomb_explosion = true;
					StatusModule::change_status_request_from_script(module_accessor, *WEAPON_GAMEWATCH_BOMB_STATUS_KIND_BURST, true); 
				}
			}
			else {
				if status_kind == *WEAPON_GAMEWATCH_BOMB_STATUS_KIND_FLY {
					*bomb_explosion = false;
				}
			}
		}
	}
}

pub fn install() {
    install_agent_frames!{
        gamewatch_functions,
        gamewatch_bomb_functions
    };
}