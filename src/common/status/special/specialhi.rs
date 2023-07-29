/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

/*   SPECIAL STATUSES   */
//Super Jump Punch Main, seems to control code regarding Super Jump Punch like moves
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_super_jump_punch_main)]
pub unsafe fn super_jump_punch_main(fighter: &mut L2CFighterCommon) {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return;
    }
    /* START OF NEW ADDITIONS */
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    //Permits Mario to be able to Wall Jump during Up Special
    if fighter_kind == *FIGHTER_KIND_MARIO {
        if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
            if (19.0..37.0).contains(&frame) {
                if !SPECIAL_WALL_JUMP {
                    if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32) {
                        if boma.is_cat_flag(Cat1::TurnDash) {
                            SPECIAL_WALL_JUMP = true;
                            fighter.change_status(FIGHTER_STATUS_KIND_WALL_JUMP.into(), true.into());
                        }
                    }
                    if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32) {
                        if boma.is_cat_flag(Cat1::TurnDash) {
                            SPECIAL_WALL_JUMP = true;
                            fighter.change_status(FIGHTER_STATUS_KIND_WALL_JUMP.into(), true.into());
                        }
                    }
                }
            }
            else {
                SPECIAL_WALL_JUMP = false;
            }
        }
    }
    /* END OF NEW ADDITIONS */
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS) {
            if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_AIR
            && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
            && MotionModule::trans_move_speed(fighter.module_accessor).y < 0.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE)
            && fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_AIR
            && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let new_status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
        fighter.change_status_req(new_status, false);
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(super_jump_punch_main);
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}