#![feature(exact_size_is_empty)]

use bevy::prelude::*;
use heron::PhysicsPlugin;
use heron::*;

struct CurrentShipEntity;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Bounce".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.2, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(spawn.system())
        .add_system(player_movement.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let station_size = Vec2::new(200.0, 50.0);
    commands
        // Spawn a bundle that contains at least a `GlobalTransform`
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(station_size),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: station_size.extend(0.0) / 2.0,
            border_radius: None,
        });

    let ship_size = Vec2::new(30.0, 30.0);

    let current_ship = CurrentShipEntity;

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(ship_size),
            material: materials.add(Color::ORANGE.into()),
            transform: Transform::from_translation(Vec3::new(-400.0, 200.0, 0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: ship_size.extend(0.0) / 4.0,
            border_radius: None,
        })
        .insert(Acceleration::from_linear(Vec3::X * 0.0))
        .insert(Velocity::from_linear(Vec3::X * 0.0))
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(current_ship);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(ship_size),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_translation(Vec3::new(-200.0, 200.0, 0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: ship_size.extend(0.0) / 4.0,
            border_radius: None,
        })
        .insert(Acceleration::from_linear(Vec3::X * 0.0))
        .insert(Velocity::from_linear(Vec3::X * 0.0));
}


fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut acceleration_query: Query<(&mut Acceleration), With<CurrentShipEntity>>,
) {
    let mut acceleration = acceleration_query.single_mut().unwrap();

    if keyboard_input.pressed(KeyCode::Left) {
        acceleration.linear = Vec3::X * -100.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        acceleration.linear = Vec3::X * 100.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        acceleration.linear = Vec3::Y * 100.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        acceleration.linear = Vec3::Y * -100.0;
    }
}