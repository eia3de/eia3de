use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(RapierConfiguration {
            gravity: Vector::y() * 20.0,
            ..def()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut cmd: Commands) {
    // Player
    cmd.spawn_bundle(RigidBodyBundle {
        position: Vec3::new(0.0, 10.0, 0.0).into(),
        ..def()
    })
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::ball(0.4).into(),
        material: ColliderMaterial {
            restitution: 0.9,
            restitution_combine_rule: CoefficientCombineRule::Max,
            ..def()
        }
        .into(),
        ..def()
    })
    .insert(ColliderPositionSync::Discrete)
    .insert(ColliderDebugRender::with_id(0));

    // Ground
    cmd.spawn_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(100.0, 0.1, 100.0).into(),
        ..def()
    })
    .insert(ColliderPositionSync::Discrete)
    .insert(ColliderDebugRender::with_id(1));

    // Light
    cmd.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            shadows_enabled: true,
            ..def()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..def()
    });

    // Camera
    cmd.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-5.0, 2.5, 0.0).looking_at(Vec3::Y, Vec3::Y),
        ..Default::default()
    });
}

fn def<T: Default>() -> T {
    Default::default()
}
