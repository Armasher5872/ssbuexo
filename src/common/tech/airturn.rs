use super::*;

#[fighter_frame_callback]
pub fn flip_air(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
		let fall_status = [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL];
		let common_attack_status = [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_AIR_LASSO, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW];
		let fighter_status_kind = [
			*FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_ATTACK, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_TURN, 
			*FIGHTER_YOSHI_STATUS_KIND_SPECIAL_AIR_LW, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N, 
			*FIGHTER_KIRBY_STATUS_KIND_CHROM_SPECIAL_N_END, *FIGHTER_KIRBY_STATUS_KIND_CHROM_SPECIAL_N_END2, *FIGHTER_KIRBY_STATUS_KIND_CHROM_SPECIAL_N_END3, *FIGHTER_KIRBY_STATUS_KIND_CHROM_SPECIAL_N_END_MAX, *FIGHTER_KIRBY_STATUS_KIND_DONKEY_SPECIAL_N_END,
			*FIGHTER_KIRBY_STATUS_KIND_DONKEY_SPECIAL_N_ATTACK, *FIGHTER_KIRBY_STATUS_KIND_EFLAME_SPECIAL_N_ATTACK, *FIGHTER_KIRBY_STATUS_KIND_EFLAME_SPECIAL_N_END, *FIGHTER_KIRBY_STATUS_KIND_ELIGHT_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_ELIGHT_SPECIAL_N_END,
			*FIGHTER_KIRBY_STATUS_KIND_GAMEWATCH_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_GANON_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END, *FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END_MDL, *FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END_MAX,
			*FIGHTER_KIRBY_STATUS_KIND_KAMUI_SPECIAL_N_BITE, *FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N2, *FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_DASH, *FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_DASH_TURN,
			*FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_MAX_DASH, *FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_MAX_DASH_END, *FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_MAX_DASH_TURN, *FIGHTER_KIRBY_STATUS_KIND_LUCINA_SPECIAL_N_END, 
			*FIGHTER_KIRBY_STATUS_KIND_LUCINA_SPECIAL_N_END_MAX, *FIGHTER_KIRBY_STATUS_KIND_MARTH_SPECIAL_N_END, *FIGHTER_KIRBY_STATUS_KIND_MARTH_SPECIAL_N_END_MAX, *FIGHTER_KIRBY_STATUS_KIND_METAKNIGHT_SPECIAL_N_SPIN, *FIGHTER_KIRBY_STATUS_KIND_METAKNIGHT_SPECIAL_N_END,
			*FIGHTER_KIRBY_STATUS_KIND_PURIN_SPECIAL_N_ROLL_AIR, *FIGHTER_KIRBY_STATUS_KIND_ROY_SPECIAL_N_END, *FIGHTER_KIRBY_STATUS_KIND_ROY_SPECIAL_N_END2, *FIGHTER_KIRBY_STATUS_KIND_ROY_SPECIAL_N_END3, *FIGHTER_KIRBY_STATUS_KIND_ROY_SPECIAL_N_END_MAX,
			*FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_HIT, *FIGHTER_KIRBY_STATUS_KIND_ZELDA_SPECIAL_N, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, 
			*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, 
			*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_S_END, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_HI_THROW, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL_AIR, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END,
			*FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A, *FIGHTER_POPO_STATUS_KIND_SPECIAL_S_PARTNER, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP_PARTNER,
			*FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_PARTNER, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END,
			*FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_S_THROW,
			*FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, 
			*FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_SHOOT,
			*FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END,
			*FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_KICK, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_HANG, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MDL, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MAX, 
			*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_RUSH, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_BLOW, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_KICK, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK, 
			*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_HIT, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK,
			*FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_APPEAR, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_END, 
			*FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE_LOOP, 
			*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_DEFOREST_HIT, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_DASH_TURN, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_MAX_DASH, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_MAX_DASH_END,
			*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_MAX_DASH_TURN, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_ATTACK,
			*FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_ATTACK, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_MOVE, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_LOOP,
			*FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_ATTACK, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH, 
			*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_SPIN_TURN, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK,
			*FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK_TURN, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2_FALL, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_BITE,
			*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_S_HOLD_END, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT,
			*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_DASH,
			*FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_LW,
			*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_FINISH, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_S_SHOULDER, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_S_LARIAT, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN,
			*FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_LOOP, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_BITE,
			*FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_THROW, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_ATTACK, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW2_COUNTER, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_ATTACK1,
			*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_ATTACK2, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_ATTACK3, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_ATTACK3_APPEND, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK,
			*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK,
			*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT_DASH, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_OVERTAKE, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_HIT,
			*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH,
			*FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_LW_ATTACK, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_HIT, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_FINISH, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N3_TURN,
			*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S2_WEAK, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S2_ATTACK, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_2, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_3,
			*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_4, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI2_ATTACK, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_AIR, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_AUTO_ATTACK, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK,
			*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW3_THROW, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_LOOP, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END_MAX, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_WEAK,
			*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_HIT, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH,
			*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_JUMP, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_LOOP, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH_END, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT,
			*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI2_JUMP, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_HIT, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HIT, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD
		];
		if TAP_WAIT[entry_id] > 1 {
			TAP_WAIT[entry_id] -= 1;
		};
		if TAP_WAIT[entry_id] == 1 {
			TAP_NUM[entry_id] = 0;
		};
		if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
			TAP_WAIT[entry_id] = 0;
		};
		if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR 
        && (fall_status.contains(&status_kind) || ((common_attack_status.contains(&status_kind) || fighter_status_kind.contains(&status_kind)) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT))) {
			if TAP_NUM[entry_id] == 0 && stick_x < -0.5 {
				TAP_NUM[entry_id] = 1;
				if TAP_WAIT[entry_id] == 0 {
					TAP_WAIT[entry_id] = TAP_MAX;
				};
			} 
            else if TAP_NUM[entry_id] == 1 && stick_x > -0.2 {
				TAP_NUM[entry_id] = 2;
			}
            else if TAP_NUM[entry_id] == 2 && stick_x < -0.5 {
				PostureModule::reverse_lr(boma);
				PostureModule::update_rot_y_lr(boma);
				TAP_WAIT[entry_id] = 1;
				TAP_NUM[entry_id] = 0;
			};
		} 
        else {
			TAP_NUM[entry_id] = 0;
		};
    }
}

pub fn install() {
    install_agent_frame_callbacks!(flip_air);
}