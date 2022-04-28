use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_rapier2d::physics::RapierConfiguration;
use bevy_rapier2d::prelude::{ColliderMaterialComponent, RigidBodyMassPropsComponent};

use crate::player::PlayerControl;
use crate::PlayerMovementSettings;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui_system);
    }
}

fn ui_system(
    mut egui_context: ResMut<EguiContext>,
    mut player_movement_settings: ResMut<PlayerMovementSettings>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut player_query: Query<
        (
            &mut RigidBodyMassPropsComponent,
            &mut ColliderMaterialComponent,
        ),
        With<PlayerControl>,
    >,
) {
    egui::Window::new("Physical Properties Tweaking").show(egui_context.ctx_mut(), |ui| {
        let mut gravity = rapier_config.gravity.y;

        let (mut player_mass_properties, mut player_collider_material) =
            player_query.get_single_mut().unwrap();
        let mut mass = player_mass_properties.mass();

        let player_movement_settings = &mut *player_movement_settings;

        fn mkslider<'a>(
            caption: &'a str,
            property: &'a mut f32,
            range: RangeInclusive<f32>,
        ) -> egui::Slider<'a> {
            egui::Slider::new(property, range).text(caption)
        }

        ui.add(mkslider(
            "Jump Height",
            &mut player_movement_settings.jump_height,
            1.0..=20.0,
        ));
        ui.add(mkslider(
            "Time To Apex",
            &mut player_movement_settings.time_to_apex,
            0.1..=5.0,
        ));

        // ui.add(mkslider("Mass", &mut mass, 0.1..=200.0));
        // ui.add(mkslider(
        //     "Friction",
        //     &mut player_collider_material.friction,
        //     0.0..=10.0,
        // ));
        ui.add(mkslider(
            "Max Speed",
            &mut player_movement_settings.max_speed,
            1.0..=100.0,
        ));
        // ui.add(mkslider(
        //     "Impulse Exponent",
        //     &mut player_movement_settings.impulse_exponent,
        //     1.0..=10.0,
        // ));
        // ui.add(mkslider(
        //     "Impulse Coefficient",
        //     &mut player_movement_settings.impulse_coefficient,
        //     1.0..=100_000.0,
        // ));
        // ui.add(mkslider(
        //     "Jump Power Coefficient",
        //     &mut player_movement_settings.jump_power_coefficient,
        //     1.0..=2000.0,
        // ));
        // ui.add(
        //     mkslider(
        //         "Jump Brake Coefficient",
        //         &mut player_movement_settings.jump_brake_coefficient,
        //         0.0..=0.1,
        //     )
        //     .logarithmic(true),
        // );
        // ui.add(mkslider(
        //     "Start Fall Before Peak",
        //     &mut player_movement_settings.start_fall_before_peak,
        //     0.0..=40.0,
        // ));
        // ui.add(mkslider(
        //     "Start of Fall Range",
        //     &mut player_movement_settings.start_of_fall_range,
        //     0.0..=40.0,
        // ));
        // ui.add(mkslider(
        //     "Start of Fall Gravity Boost",
        //     &mut player_movement_settings.start_of_fall_gravity_boost,
        //     0.0..=100.0,
        // ));
        // ui.add(
        //     mkslider(
        //         "Fall Boost Coefficient",
        //         &mut player_movement_settings.fall_boost_coefficient,
        //         1.0..=2.0,
        //     )
        //     .logarithmic(true),
        // );
        // ui.add(mkslider(
        //     "Stood On Time Coefficient",
        //     &mut player_movement_settings.stood_on_time_coefficient,
        //     1.0..=100.0,
        // ));
        // ui.add(mkslider(
        //     "Uphill Move Exponent",
        //     &mut player_movement_settings.uphill_move_exponent,
        //     0.01..=200.0,
        // ));
        // ui.add(mkslider(
        //     "Downhill Brake Exponent",
        //     &mut player_movement_settings.downhill_brake_exponent,
        //     0.01..=200.0,
        // ));

        rapier_config.gravity.y = -(2.0 * player_movement_settings.jump_height)
            / player_movement_settings.time_to_apex.powf(2.0);
        player_movement_settings.jump_power_coefficient =
            (2.0 * -rapier_config.gravity.y * player_movement_settings.jump_height).sqrt();
        player_mass_properties.local_mprops.set_mass(mass, true);
    });
}

fn ui_system2(
    mut egui_context: ResMut<EguiContext>,
    mut player_movement_settings: ResMut<PlayerMovementSettings>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
    mut player_query: Query<
        (
            &mut RigidBodyMassPropsComponent,
            &mut ColliderMaterialComponent,
        ),
        With<PlayerControl>,
    >,
) {
    egui::Window::new("Physical Properties Tweaking").show(egui_context.ctx_mut(), |ui| {
        let mut gravity = -rapier_configuration.gravity.y;

        let (mut player_mass_properties, mut player_collider_material) =
            player_query.get_single_mut().unwrap();
        let mut mass = player_mass_properties.mass();

        let player_movement_settings = &mut *player_movement_settings;

        fn mkslider<'a>(
            caption: &'a str,
            property: &'a mut f32,
            range: RangeInclusive<f32>,
        ) -> egui::Slider<'a> {
            egui::Slider::new(property, range).text(caption)
        }

        ui.add(mkslider("Gravity", &mut gravity, 0.1..=20.0));
        ui.add(mkslider("Mass", &mut mass, 0.1..=200.0));
        ui.add(mkslider(
            "Friction",
            &mut player_collider_material.friction,
            0.0..=10.0,
        ));
        ui.add(mkslider(
            "Max Speed",
            &mut player_movement_settings.max_speed,
            1.0..=100.0,
        ));
        ui.add(mkslider(
            "Impulse Exponent",
            &mut player_movement_settings.impulse_exponent,
            1.0..=10.0,
        ));
        ui.add(mkslider(
            "Impulse Coefficient",
            &mut player_movement_settings.impulse_coefficient,
            1.0..=100_000.0,
        ));
        ui.add(mkslider(
            "Jump Power Coefficient",
            &mut player_movement_settings.jump_power_coefficient,
            1.0..=2000.0,
        ));
        ui.add(
            mkslider(
                "Jump Brake Coefficient",
                &mut player_movement_settings.jump_brake_coefficient,
                0.0..=0.1,
            )
            .logarithmic(true),
        );
        ui.add(mkslider(
            "Start Fall Before Peak",
            &mut player_movement_settings.start_fall_before_peak,
            0.0..=40.0,
        ));
        ui.add(mkslider(
            "Start of Fall Range",
            &mut player_movement_settings.start_of_fall_range,
            0.0..=40.0,
        ));
        ui.add(mkslider(
            "Start of Fall Gravity Boost",
            &mut player_movement_settings.start_of_fall_gravity_boost,
            0.0..=100.0,
        ));
        ui.add(
            mkslider(
                "Fall Boost Coefficient",
                &mut player_movement_settings.fall_boost_coefficient,
                1.0..=2.0,
            )
            .logarithmic(true),
        );
        ui.add(mkslider(
            "Stood On Time Coefficient",
            &mut player_movement_settings.stood_on_time_coefficient,
            1.0..=100.0,
        ));
        ui.add(mkslider(
            "Uphill Move Exponent",
            &mut player_movement_settings.uphill_move_exponent,
            0.01..=200.0,
        ));
        ui.add(mkslider(
            "Downhill Brake Exponent",
            &mut player_movement_settings.downhill_brake_exponent,
            0.01..=200.0,
        ));

        rapier_configuration.gravity.y = -gravity;
        player_mass_properties.local_mprops.set_mass(mass, true);
    });
}
