use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::render::RapierRenderPlugin;
use move_vis::MoveVisPlugin;

use crate::arena::ArenaPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UiPlugin;

mod arena;
mod player;
mod ui;

const jump_height: f32 = 4.0;

// how long does it take to reach the maximum height of a jump?
// note: if "jump_power_coefficient" is not a multiple of "g" the maximum height is reached between frames
// second
const time_to_apex: f32 = 0.4;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(ArenaPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(MoveVisPlugin)
        .insert_resource(PlayerMovementSettings {
            jump_height,
            time_to_apex,
            max_speed: 2.0,
            impulse_exponent: 4.0,
            impulse_coefficient: 40_000.0,
            jump_power_coefficient: 0.0,
            jump_brake_coefficient: 0.02,
            start_fall_before_peak: 10.0,
            start_of_fall_range: 10.0,
            start_of_fall_gravity_boost: 30.0,
            fall_boost_coefficient: 1.06,
            stood_on_time_coefficient: 10.0,
            uphill_move_exponent: 0.5,
            downhill_brake_exponent: 1.0,
        })
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut player_movement_settings: ResMut<PlayerMovementSettings>,
) {
    // what is the gravity that would allow jumping to a given height?
    rapier_config.gravity.y = -(2.0 * jump_height) / time_to_apex.powf(2.0);

    // what is the initial jump velocity?
    player_movement_settings.jump_power_coefficient =
        (2.0 * rapier_config.gravity.y.abs() * jump_height).sqrt();

    let mut camera = OrthographicCameraBundle::new_2d();
    let zoom = 20.0;
    camera.transform.scale.x /= zoom;
    camera.transform.scale.y /= zoom;
    camera.transform.translation.x += 7.5;
    camera.transform.translation.y += 9.0;
    commands.spawn_bundle(camera);
}

#[derive(Debug)]
pub struct PlayerMovementSettings {
    // metre
    pub jump_height: f32,
    // second
    pub time_to_apex: f32,
    pub max_speed: f32,
    pub impulse_exponent: f32,
    pub impulse_coefficient: f32,
    pub jump_power_coefficient: f32,
    pub jump_brake_coefficient: f32,
    pub start_fall_before_peak: f32,
    pub start_of_fall_range: f32,
    pub start_of_fall_gravity_boost: f32,
    pub fall_boost_coefficient: f32,
    pub stood_on_time_coefficient: f32,
    pub uphill_move_exponent: f32,
    pub downhill_brake_exponent: f32,
}
