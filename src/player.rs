use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::na::Vector2;
use bevy_rapier2d::prelude::*;

use move_vis::TrackMovement;

use crate::PlayerMovementSettings;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_system(control_player);
    }
}

#[derive(Debug)]
enum JumpStatus {
    CanJump,
    InitiateJump,
    GoingUp,
    StoppingUp,
    GoingDown,
}

fn setup_player(mut commands: Commands) {
    let mut cmd = commands.spawn();
    cmd.insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Dynamic.into(),
        position: point![0.0, 0.0].into(),
        mass_properties: RigidBodyMassProps {
            flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });
    cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(0.25, 1.0).into(),
        ..Default::default()
    });
    cmd.insert(ColliderPositionSync::Discrete);
    cmd.insert(ColliderDebugRender::with_id(2));
    cmd.insert(TrackMovement::default());
    cmd.insert(PlayerControl {
        mid_jump: false,
        last_stood_on: vector![0.0, 1.0],
        stood_on_potential: 0.0,
    });
}

#[derive(Component)]
pub struct PlayerControl {
    mid_jump: bool,
    last_stood_on: Vector2<f32>,
    stood_on_potential: f32,
}

fn control_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(
        Entity,
        &mut RigidBodyVelocityComponent,
        &mut RigidBodyForcesComponent,
        &RigidBodyMassPropsComponent,
        &mut PlayerControl,
    )>,
    player_movement_settings: Res<PlayerMovementSettings>,
    narrow_phase: Res<NarrowPhase>,
) {
    let is_jumping = input.pressed(KeyCode::Space);
    let mut target_speed: f32 = 0.0;

    if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
        target_speed -= 1.0;
    }

    if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
        target_speed += 1.0;
    }

    // target_speed *= player_movement_settings.max_speed;

    for (player_entity, mut velocity, mut rb_force, mass_props, mut player_control) in
        query.iter_mut()
    {
        let standing_on = narrow_phase
            .contacts_with(player_entity.handle())
            .filter(|contact| contact.has_any_active_contact)
            .flat_map(|contact| {
                contact.manifolds.iter().filter_map(|contact_manifold| {
                    let player_handle = player_entity.handle();
                    if contact_manifold.data.rigid_body1 == Some(player_handle) {
                        Some(-contact_manifold.data.normal)
                    } else if contact_manifold.data.rigid_body2 == Some(player_handle) {
                        Some(contact_manifold.data.normal)
                    } else {
                        None
                    }
                })
            })
            .max_by_key(|normal| float_ord::FloatOrd(normal.dot(&vector![0.0, 1.0])));

        // determine jump status of player
        let jump_status = (|| {
            if let Some(standing_on) = standing_on {
                // player_control.last_stood_on = standing_on;

                // if 0.0 < standing_on.dot(&vector![0.0, 1.0]) {
                if is_jumping {
                    return JumpStatus::InitiateJump;
                }
                return JumpStatus::CanJump;
                // }
            }

            if 0.0 <= velocity.linvel.y {
                if is_jumping && player_control.mid_jump {
                    JumpStatus::GoingUp
                } else {
                    JumpStatus::StoppingUp
                }
            } else {
                JumpStatus::GoingDown
            }
        })();

        match jump_status {
            JumpStatus::GoingDown => rb_force.gravity_scale = 5.0,
            _ => rb_force.gravity_scale = 1.0
        };

        match jump_status {
            JumpStatus::CanJump => {
                player_control.mid_jump = false;
            }
            JumpStatus::InitiateJump => {
                player_control.mid_jump = true;
                velocity.apply_impulse(
                    mass_props,
                    // vector![0.0, 1.0] * player_movement_settings.init_jump_velocity * time.delta().as_secs_f32(),
                    vector![0.0, 1.0] * player_movement_settings.jump_power_coefficient,
                );
            }
            JumpStatus::GoingUp => {
                player_control.mid_jump = true;
            }
            JumpStatus::StoppingUp => {
                player_control.mid_jump = false;
                velocity.linvel.y *= player_movement_settings
                    .jump_brake_coefficient
                    .powf(time.delta().as_secs_f32());
                if velocity.linvel.y < player_movement_settings.start_fall_before_peak {
                    velocity.linvel.y -= player_movement_settings.start_of_fall_gravity_boost
                        * time.delta().as_secs_f32();
                }
            }
            JumpStatus::GoingDown => {
                // if -player_movement_settings.start_of_fall_range < velocity.linvel.y {
                //     // reminder: linvel.y is negative here
                //     velocity.linvel.y -= player_movement_settings.start_of_fall_gravity_boost
                //         * time.delta().as_secs_f32();
                // } else {
                //     velocity.linvel.y *= player_movement_settings
                //         .fall_boost_coefficient
                //         .powf(time.delta().as_secs_f32());
                // }
                player_control.mid_jump = false;
            }
        }

        let mut up_now = vector![0.0, 1.0];
        // up_now = (1.0 - player_control.stood_on_potential) * up_now
        //     + player_control.stood_on_potential * player_control.last_stood_on;

        let movement_vector = Isometry::rotation(-std::f32::consts::FRAC_PI_2) * up_now;

        dbg!(&movement_vector);
        // let current_speed =
        //     velocity.linvel.dot(&movement_vector) / player_movement_settings.max_speed;

        // if (0.0 < target_speed && target_speed <= current_speed)
        //     || (target_speed < 0.0 && current_speed <= target_speed)
        // {
        //     continue;
        // }

        // let impulse = target_speed - current_speed;
        // dbg!(&impulse);
        // let impulse = if 1.0 < impulse.abs() {
        //     impulse.signum()
        // } else {
        //     impulse.signum()
        //         * impulse
        //             .abs()
        //             .powf(player_movement_settings.impulse_exponent)
        // };
        // let mut impulse = movement_vector
        //     * time.delta().as_secs_f32()
        //     * player_movement_settings.impulse_coefficient
        //     * impulse;
        // let uphill = impulse.normalize().dot(&vector![0.0, 1.0]);
        // if 0.01 <= uphill {
        //     let efficiency = if target_speed.signum() as i32 == current_speed.signum() as i32 {
        //         player_movement_settings.uphill_move_exponent
        //     } else {
        //         player_movement_settings.downhill_brake_exponent
        //     };
        //     impulse *= 1.0 - uphill.powf(efficiency);
        // }
        // velocity.apply_impulse(mass_props, impulse);
    }
}
