use {
    crate::functions::{
        ext::*,
        var::{
            captain::*,
            consts::*,
            globals::*,
            variables::*,
        }
    },
    smash::{
        app::{
            lua_bind::*,
            sv_information
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod acmd;
mod opff;
mod status;

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
}