use super::*;

unsafe extern "C" fn pfushigisou_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let lr = PostureModule::lr(boma);
    //Squirtle's Rain Dance Region Detection
    let point_offset_x1 = lr * (RAIN_DANCE_X1[entry_id] - pos_x);
    let point_offset_x2 = lr * (RAIN_DANCE_X2[entry_id] - pos_x);
    let point_offset_y1 = RAIN_DANCE_Y1[entry_id] - pos_y;
    if (pos_x <= RAIN_DANCE_X2[entry_id] && pos_x >= RAIN_DANCE_X1[entry_id]) && (pos_y >= RAIN_DANCE_Y1[entry_id] && pos_y <= RAIN_DANCE_Y2[entry_id]) && IN_RAIN_DANCE[entry_id] != true {
        IN_RAIN_DANCE[entry_id] = true;
    }
    else {
        IN_RAIN_DANCE[entry_id] = false;
    }
    //Rain Dance Effects
    if RAIN_DANCE_ACTIVE[entry_id] == true {
        RAIN_DANCE_FRAME[entry_id] -= 1.0;
        if RAIN_DANCE_FRAME[entry_id] <= 600.0 && RAIN_DANCE_FRAME[entry_id] >= 598.0 {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_ground_shockwave"), false, false);
        }
        else if RAIN_DANCE_FRAME[entry_id] < 598.0 && RAIN_DANCE_FRAME[entry_id] % 30.0 == 0.0 {
            macros::EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), point_offset_x1, point_offset_y1, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), point_offset_x2, point_offset_y1, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), point_offset_x1 - (12.0*lr), point_offset_y1, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), point_offset_x2 + (12.0*lr), point_offset_y1, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
        if RAIN_DANCE_FRAME[entry_id] <= 0.0 {
            RAIN_DANCE_ACTIVE[entry_id] = false;
        }
    }
    if RAIN_DANCE_ACTIVE[entry_id] == false {
        RAIN_DANCE_FRAME[entry_id] = -1.0;
        RAIN_DANCE_X1[entry_id] = 0.0;
        RAIN_DANCE_X2[entry_id] = 0.0;
        RAIN_DANCE_Y1[entry_id] = 0.0;
        RAIN_DANCE_Y2[entry_id] = 0.0;
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_ground_shockwave"), false, false);
    }
    //Seed Bomb
    let itemmanager = smash2::app::ItemManager::instance().unwrap();
    let bomb_count = smash2::app::ItemManager::get_num_of_ownered_item(itemmanager, (*boma).battle_object_id, smash2::app::ItemKind::Deku);
    if bomb_count == 0 {
        PFUSHIGISOU_IS_ACTIVE_BOMB[entry_id] = false;
    }
    println!("Is Active Bomb: {}", PFUSHIGISOU_IS_ACTIVE_BOMB[entry_id]);
}

unsafe extern "C" fn pfushigisou_init(fighter: &mut L2CFighterCommon) {
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
    //Ivysaur
    PFUSHIGISOU_IS_ACTIVE_BOMB[entry_id] = false;
}

pub fn install() {
    Agent::new("pfushigisou")
    .on_start(pfushigisou_init)
    .on_line(Main, pfushigisou_frame)
    .install()
    ;
}