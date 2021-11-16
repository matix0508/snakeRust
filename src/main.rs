mod game;
use rand::prelude::random;
use bevy::core::FixedTimestep;

use bevy::prelude::*;
use bevy::render::pass::ClearColor;

/// function to setup the scene
/// gets called in main
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    //Including Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(game::Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        food_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
    });
}

///Main function
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Snake".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(game::spawn_snake.system()))
        .add_system(game::snake_movement.system())
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(game::position_translation.system())
                .with_system(game::size_scaling.system())
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(game::food_spawner.system()),
        )
        .add_plugins(DefaultPlugins)
        .run();
}
