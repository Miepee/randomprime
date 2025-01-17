pub mod res_id;

mod ancs;
mod anim;
mod bnr;
mod cmdl;
mod dol;
mod dumb;
mod evnt;
mod font;
mod frme;
mod gc_disc;
mod hint;
mod mapa;
mod mapw;
pub mod mlvl;
mod mrea;
mod pak;
mod ctwk;
mod part;
mod savw;
mod scan;
mod scly;
mod strg;
mod thp;
mod txtr;

pub mod scly_props
{
    // http://www.metroid2002.com/retromodding/wiki/User:Parax0/Sandbox
    pub mod actor;
    pub mod actor_key_frame;
    pub mod ball_trigger;
    pub mod camera;
    pub mod camera_blur_keyframe;
    pub mod camera_filter_keyframe;
    pub mod camera_hint_trigger;
    pub mod camera_hint;
    pub mod counter;
    pub mod damageable_trigger;
    pub mod distance_fog;
    pub mod dock;
    pub mod door;
    pub mod effect;
    pub mod grapple_point;
    pub mod hud_memo;
    pub mod memory_relay;
    pub mod new_camera_shaker;
    pub mod pickup;
    pub mod pickup_generator;
    pub mod platorm;
    pub mod point_of_interest;
    pub mod player_actor;
    pub mod player_hint;
    pub mod relay;
    pub mod snake_weed_swarm;
    pub mod sound;
    pub mod spawn_point;
    pub mod special_function;
    pub mod streamed_audio;
    pub mod timer;
    pub mod trigger;
    pub mod water;
    pub mod waypoint;
    pub mod world_transporter;

    // bosses
    pub mod beetle;
    pub mod drone;
    pub mod new_intro_boss;
    pub mod actor_contraption;
    pub mod flaahgra;
    pub mod ice_sheegoth;
    pub mod thardus;
    pub mod elite_pirate;
    pub mod omega_pirate;
    pub mod ridley_v1;
    pub mod ridley_v2;
    pub mod metroidprimestage1;
    pub mod metroidprimestage2;

    pub mod structs;

    pub use self::actor::*;
    pub use self::actor_key_frame::*;
    pub use self::ball_trigger::*;
    pub use self::camera::*;
    pub use self::camera_blur_keyframe::*;
    pub use self::camera_filter_keyframe::*;
    pub use self::camera_hint_trigger::*;
    pub use self::camera_hint::*;
    pub use self::counter::*;
    pub use self::damageable_trigger::*;
    pub use self::distance_fog::*;
    pub use self::dock::*;
    pub use self::door::*;
    pub use self::effect::*;
    pub use self::grapple_point::*;
    pub use self::hud_memo::*;
    pub use self::memory_relay::*;
    pub use self::new_camera_shaker::*;
    pub use self::pickup::*;
    pub use self::pickup_generator::*;
    pub use self::platorm::*;
    pub use self::point_of_interest::*;
    pub use self::player_actor::*;
    pub use self::player_hint::*;
    pub use self::relay::*;
    pub use self::snake_weed_swarm::*;
    pub use self::sound::*;
    pub use self::spawn_point::*;
    pub use self::special_function::*;
    pub use self::streamed_audio::*;
    pub use self::timer::*;
    pub use self::trigger::*;
    pub use self::water::*;
    pub use self::waypoint::*;
    pub use self::world_transporter::*;

    // bosses
    pub use self::beetle::*;
    pub use self::drone::*;
    pub use self::new_intro_boss::*;
    pub use self::actor_contraption::*;
    pub use self::flaahgra::*;
    pub use self::ice_sheegoth::*;
    pub use self::thardus::*;
    pub use self::elite_pirate::*;
    pub use self::omega_pirate::*;
    pub use self::ridley_v1::*;
    pub use self::ridley_v2::*;
    pub use self::metroidprimestage1::*;
    pub use self::metroidprimestage2::*;
}

pub use scly_props::structs as scly_structs;
pub use scly_props::actor::*;
pub use scly_props::actor_key_frame::*;
pub use scly_props::ball_trigger::*;
pub use scly_props::camera::*;
pub use scly_props::camera_blur_keyframe::*;
pub use scly_props::camera_filter_keyframe::*;
pub use scly_props::camera_hint_trigger::*;
pub use scly_props::camera_hint::*;
pub use scly_props::damageable_trigger::*;
pub use scly_props::distance_fog::*;
pub use scly_props::dock::*;
pub use scly_props::door::*;
pub use scly_props::effect::*;
pub use scly_props::grapple_point::*;
pub use scly_props::hud_memo::*;
pub use scly_props::memory_relay::*;
pub use scly_props::new_camera_shaker::*;
pub use scly_props::pickup_generator::*;
pub use scly_props::pickup::*;
pub use scly_props::platorm::*;
pub use scly_props::point_of_interest::*;
pub use scly_props::player_actor::*;
pub use scly_props::player_hint::*;
pub use scly_props::relay::*;
pub use scly_props::snake_weed_swarm::*;
pub use scly_props::sound::*;
pub use scly_props::spawn_point::*;
pub use scly_props::special_function::*;
pub use scly_props::streamed_audio::*;
pub use scly_props::timer::*;
pub use scly_props::trigger::*;
pub use scly_props::water::*;
pub use scly_props::waypoint::*;
pub use scly_props::world_transporter::*;

// bosses
pub use scly_props::beetle::*;
pub use scly_props::drone::*;
pub use scly_props::new_intro_boss::*;
pub use scly_props::actor_contraption::*;
pub use scly_props::flaahgra::*;
pub use scly_props::ice_sheegoth::*;
pub use scly_props::thardus::*;
pub use scly_props::elite_pirate::*;
pub use scly_props::omega_pirate::*;
pub use scly_props::ridley_v1::*;
pub use scly_props::ridley_v2::*;
pub use scly_props::metroidprimestage1::*;
pub use scly_props::metroidprimestage2::*;

pub use res_id::ResId;

pub use anim::*;
pub use ancs::*;
pub use bnr::*;
pub use cmdl::*;
pub use dol::*;
pub use dumb::*;
pub use evnt::*;
pub use font::*;
pub use frme::*;
pub use gc_disc::*;
pub use hint::*;
pub use mapa::*;
pub use mapw::*;
pub use mlvl::*;
pub use mrea::*;
pub use pak::*;
pub use ctwk::*;
pub use part::*;
pub use savw::*;
pub use scan::*;
pub use scly::*;
pub use strg::*;
pub use thp::*;
pub use txtr::*;
