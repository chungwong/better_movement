use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_arena);
    }
}

fn setup_arena(mut commands: Commands) {
    for (x, y, half_width, half_height, rotation) in [
        (0.0, -4.0, 15.0, 1.0, 0.0),
        (7.0, 2.0, 2.0, 0.5, 0.0),
        (-7.0, 0.0, 2.0, 0.5, 0.0),
        (20.0, 2.5, 10.0, 1.0, std::f32::consts::FRAC_PI_4),
    ] {
        let mut cmd = commands.spawn();
        cmd.insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: Isometry::new(vector![x, y], rotation).into(),
            ..Default::default()
        });
        cmd.insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(half_width, half_height).into(),
            ..Default::default()
        });
        cmd.insert(ColliderPositionSync::Discrete);
        cmd.insert(ColliderDebugRender::with_id(1));
    }
}
