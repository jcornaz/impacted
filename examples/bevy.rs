use std::f32::consts;

use bevy::prelude::*;
use bevy::transform::TransformSystem;

#[derive(Debug, Component, Deref, DerefMut)]
struct CollisionShape(impacted::CollisionShape);

/// As simple "tag" component to mark the entity controlled by keyboard
/// (Nothing specific about collision detection)
#[derive(Component)]
struct Controlled;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_system(control_shape)
        // Collision detection
        .add_system_to_stage(
            CoreStage::PostUpdate,
            update_shape_transforms // First update transforms
                .chain(update_color) // Then update the colors
                .after(TransformSystem::TransformPropagate), // Better to consider the up-to-date transforms
        )
        .run();
}

/// Initialize the "game"
fn startup(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Left shape (controlled)
    commands
        // Add a sprite so we can see it (nothing specific about collision detection)
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.0)),
                color: Color::BLUE,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-200., 0.0, 0.0)),
            ..Default::default()
        })
        .insert_bundle((
            // Add the collision shape
            CollisionShape(impacted::CollisionShape::new_rectangle(100.0, 100.0)),
            // Mark this shape as the one being controlled (nothing specific to collision detection)
            Controlled,
        ));

    // Right shape (static)
    commands
        // Add a sprite so we can see it (nothing specific about collision detection)
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.0)),
                color: Color::BLUE,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(200., 0.0, 0.0)),
            ..Default::default()
        })
        // Add the collision shape
        .insert(CollisionShape(impacted::CollisionShape::new_rectangle(
            100.0, 100.0,
        )));
}

/// Update the `CollisionShape` transform if the `GlobalTransform` has changed
fn update_shape_transforms(
    mut shapes: Query<(&mut CollisionShape, &GlobalTransform), Changed<GlobalTransform>>,
) {
    for (mut shape, transform) in shapes.iter_mut() {
        let (scale, rotation, translation) = transform.to_scale_rotation_translation();
        shape.set_transform(impacted::Transform::from_scale_angle_translation(
            scale.truncate(),
            angle_2d_from_quat(rotation),
            translation.truncate(),
        ));
    }
}

fn angle_2d_from_quat(quat: Quat) -> f32 {
    if quat.is_near_identity() {
        return 0.0;
    }
    let projected = quat.to_scaled_axis().project_onto(Vec3::Z);
    let angle = projected.length();
    if projected.z < 0.0 {
        -angle
    } else {
        angle
    }
}

/// Detect collision and update shape colors
///
/// Notice, that it only looks at the shapes that have moved to avoid unnecessary collision test
///
/// It still tests each moved shape against each other shape as this library only provide narrow-phase collision test.
/// For many small games it should be fine.
/// For bigger games, you may consider to pair it with a broad-phase (like [bvh-arena] or [broccoli])
/// to reduce the number of collision test to perform.
///
/// Also, remember that this implementation is quite generic,
/// and it might be simplified for your use-case.
/// Example: check for collision between each enemy that has moved and the player.
/// You may even have many of this kind of system for different aspect of the game logic, and bevy can run them in parallel!
///
/// [bvh-arena]: https://crates.io/crates/bvh-arena
/// [broccoli]: https://crates.io/crates/broccoli
fn update_color(
    mut all_shapes: Query<(Entity, &mut Sprite, &CollisionShape)>,
    moved_shapes: Query<(Entity, &CollisionShape), Changed<CollisionShape>>,
) {
    for (moved_entity, moved_shape) in moved_shapes.iter() {
        let mut is_collided = false;
        for (other_entity, mut other_sprite, other_shape) in all_shapes.iter_mut() {
            if other_entity == moved_entity {
                continue; // Don't test collision with self
            }

            // Test for collision, and update the other shape color
            if moved_shape.is_collided_with(other_shape) {
                other_sprite.color = Color::RED;
                is_collided = true;
            } else {
                other_sprite.color = Color::BLUE;
            }
        }

        // Update the moved shape color
        let (_, mut sprite, _) = all_shapes.get_mut(moved_entity).unwrap();
        sprite.color = if is_collided { Color::RED } else { Color::BLUE }
    }
}

/// Simple control system to move the shape with the arrows keys, and rotate with `A` and `D`
/// (Nothing specific about collision detection here)
fn control_shape(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform, With<Controlled>>,
) {
    const LIN_SPEED: f32 = 200.0;
    const ANG_SPEED: f32 = consts::FRAC_PI_2;

    let lin_delta = Vec3::new(
        get_input_axis(&input, KeyCode::Left, KeyCode::Right),
        get_input_axis(&input, KeyCode::Down, KeyCode::Up),
        0.0,
    )
    .normalize_or_zero()
        * (LIN_SPEED * time.delta_seconds());

    let ang_delta = Quat::from_axis_angle(
        Vec3::Z,
        get_input_axis(&input, KeyCode::D, KeyCode::A) * ANG_SPEED * time.delta_seconds(),
    );

    for mut transform in transforms.iter_mut() {
        transform.translation += lin_delta;
        transform.rotation *= ang_delta;
    }

    fn get_input_axis(input: &Input<KeyCode>, negative: KeyCode, positive: KeyCode) -> f32 {
        let mut res = 0.0;
        if input.pressed(positive) {
            res += 1.0;
        }
        if input.pressed(negative) {
            res -= 1.0;
        }
        res
    }
}
