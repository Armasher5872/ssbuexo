use super::*;

unsafe extern "C" fn diddy_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let boma_opponent1 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
    let status_kind_opponent1 = StatusModule::status_kind(boma_opponent1);
    let boma_opponent2 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(2));
    let status_kind_opponent2 = StatusModule::status_kind(boma_opponent2);
    let boma_opponent3 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(3));
    let status_kind_opponent3 = StatusModule::status_kind(boma_opponent3);
    let boma_opponent4 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(4));
    let status_kind_opponent4 = StatusModule::status_kind(boma_opponent4);
    let boma_opponent5 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(5));
    let status_kind_opponent5 = StatusModule::status_kind(boma_opponent5);
    let boma_opponent6 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(6));
    let status_kind_opponent6 = StatusModule::status_kind(boma_opponent6);
    let boma_opponent7 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(7));
    let status_kind_opponent7 = StatusModule::status_kind(boma_opponent7);
    let banana_id = WorkModule::get_int(boma, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_WORK_INT_BANANA_ID);
    if prev_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW
    && BANANA_EXIST[entry_id] == false {
        BANANA_EXIST[entry_id] = true;
    }
    if status_kind_opponent1 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent2 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent3 == *FIGHTER_STATUS_KIND_SLIP 
    || status_kind_opponent4 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent5 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent6 == *FIGHTER_STATUS_KIND_SLIP 
    || status_kind_opponent7 == *FIGHTER_STATUS_KIND_SLIP && BANANA_EXIST[entry_id] == true {
        WorkModule::set_int(boma, banana_id, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_WORK_INT_BANANA_ID);
        BANANA_EXIST[entry_id] = false;
        StatusModule::change_status_request_from_script(boma, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_LW_LAUGH, false);
    }
    println!("Is Banana Exist {:?}", BANANA_EXIST[entry_id]);
}

unsafe extern "C" fn diddy_init(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    //Universal
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
    WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    SPECIAL_WALL_JUMP = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    LAST_DAMAGE[entry_id] = 0.0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SIZE2[entry_id] = 0.0;
    SIZE3[entry_id] = 0.0;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    //Diddy
    BANANA_EXIST[entry_id] = false;
}

pub fn install() {
    Agent::new("diddy")
    .on_start(diddy_init)
    .on_line(Main, diddy_frame)
    .install()
    ;
}