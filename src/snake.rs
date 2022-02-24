use bevy::prelude::*;

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Snake;

const VELOCITY: f32 = 5.;

impl Snake {
    pub fn spawn(commands: &mut Commands) {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.1, 0.1),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    scale: Vec3::new(25., 25., 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Snake);
    }

    pub fn handle_keyboard(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<&mut Transform, With<Snake>>,
    ) {
        for mut trans in query.iter_mut() {
            let translation = &mut trans.translation;

            if keyboard_input.pressed(KeyCode::W) {
                translation.y += VELOCITY;
            }

            if keyboard_input.pressed(KeyCode::S) {
                translation.y -= VELOCITY;
            }

            if keyboard_input.pressed(KeyCode::D) {
                translation.x += VELOCITY;
            }

            if keyboard_input.pressed(KeyCode::A) {
                translation.x -= VELOCITY;
            }
        }
    }
}
