//Custom Universal Consts
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START: i32 = 495;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE: i32 = 496;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL: i32 = 497;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING: i32 = 498;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY: i32 = 499;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED: i32 = 500;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER: i32 = 501;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK: i32 = 502; //Tracks if a fighter used a certain special move in the air
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD: i32 = 503;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED: i32 = 504;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT: i32 = 505;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT: i32 = 506;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE: i32 = 507;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK: i32 = 508;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW: i32 = 509;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH: i32 = 510;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO: i32 = 511; //Returns false for exactly one frame after is_ready_go becomes true, used to initiate certain events exactly once at the start of a match
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL: i32 = 512;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE: i32 = 513;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE: i32 = 514;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE: i32 = 515;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE: i32 = 516;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS: i32 = 517; //Flags when you just used a Final Smash in Special Smash
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH_DONE: i32 = 518;
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL: i32 = 519;
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_R: i32 = 520;
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_G: i32 = 521;
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_B: i32 = 522;
pub const FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT: i32 = 523; //Tracks if a player got hit during One-Hit mode
pub const FIGHTER_INSTANCE_WORK_ID_INT_MASHING: i32 = 524;
pub const FIGHTER_INSTANCE_WORK_ID_INT_PARRIED: i32 = 525;
pub const FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER: i32 = 526;
pub const FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER: i32 = 527;
pub const FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX: i32 = 528;
pub const FRAME_FALL: f32 = 34.0;
pub const FRAME_LAND: f32 = 52.0;
pub const NONE_VECTOR: smash::phx::Vector3f = smash::phx::Vector3f {x: 0.0, y: 0.0, z: 0.0};