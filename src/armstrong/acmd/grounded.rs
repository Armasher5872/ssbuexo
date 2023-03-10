#![allow(unused_macros)]
use {
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
        },
        lua2cpp::L2CAgentBase,
        lib::lua_const::*,
        phx::Hash40,
    },
    smashline::*,
    smash_script::*,
};

//Down Taunt ACMD
#[acmd_script( agent = "ganon", scripts = ["game_appeallwr", "game_appeallwl"], category = ACMD_GAME)]
unsafe fn ssbuexo_armstrong_down_taunt_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
        HitModule::set_check_catch(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 80.0);
    if macros::is_excute(fighter) {
        HitModule::set_check_catch(fighter.module_accessor, true, 0);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
}

//Sword Taunt ACMD (Removes the sword from the animation)
#[acmd_script( agent = "ganon_sword", script = "game_appeallw", category = ACMD_GAME)]
unsafe fn ssbuexo_armstrong_sword_taunt_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, smash::app::ArticleOperationTarget(0));
    }
}

//Jab ACMD
#[acmd_script( agent = "ganon", script = "game_attack11", category = ACMD_GAME)]
unsafe fn ssbuexo_armstrong_jab_acmd(fighter: &mut L2CAgentBase) 
{
    MotionModule::set_rate(fighter.module_accessor, 1.75);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 1.2);
        DamageModule::set_reaction_mul(fighter.module_accessor, 0.65);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 74, 0, 41, 4.4, 0.0, 9.5, 11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 74, 0, 41, 5.0, 0.0, 9.5, 19.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 361, 74, 0, 41, 3.5, 0.0, 9.5, 7.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
}

//Jab Effect
#[acmd_script( agent = "ganon", script = "effect_attack11", category = ACMD_EFFECT)]
unsafe fn ssbuexo_armstrong_jab_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 13.3, -0.5, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.88, 0.35, 0.13);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, 1.5);
    }
}

//Dash Attack ACMD
#[acmd_script( agent = "ganon", script = "game_attackdash", category = ACMD_GAME)]
unsafe fn ssbuexo_armstrong_dash_attack_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 6.0);
        DamageModule::set_reaction_mul(fighter.module_accessor, 0.65);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 70, 85, 0, 50, 7.0, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 80, 60, 0, 45, 4.0, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.3);
    }
    wait(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_armstrong_down_taunt_acmd,
        ssbuexo_armstrong_sword_taunt_acmd,
        ssbuexo_armstrong_jab_acmd,
        ssbuexo_armstrong_jab_effect,
        ssbuexo_armstrong_dash_attack_acmd
    );
}