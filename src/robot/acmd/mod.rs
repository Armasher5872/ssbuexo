use {
    crate::functions::{
        ext::*,
        var::variables::*,
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

mod tilts;
mod smashes;
mod aerials;
mod throws;
mod specials;

pub fn install() {
  tilts::install();
  smashes::install();
  aerials::install();
  throws::install();
  specials::install();
}