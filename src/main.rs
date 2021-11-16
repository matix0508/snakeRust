mod snake;

use bevy::prelude::*;

/// function to setup the scene
/// gets called in main
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    //Including Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(snake::Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
}

///Main function
fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(snake::spawn_snake.system()))
        .add_system(snake::snake_movement.system())
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(snake::grid::position_translation.system())
                .with_system(snake::grid::size_scaling.system())
        )
        .add_plugins(DefaultPlugins)
        .run();
}
