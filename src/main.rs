use bevy::prelude::*;
use rand::Rng;
use snake::{Collider, Snake};

mod snake;

#[derive(Component)]
struct Score(u32);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/FiraCode-SemiBold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(Score(0));

    spawn_block(&mut commands);

    Snake::spawn(&mut commands);
}

fn score_system(score: Res<Score>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("{}", score.0);
}

fn change_title(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title(format!("Snake"));
}

fn collides(o: (Vec2, Vec2), t: (Vec2, Vec2)) -> bool {
    //rect1.x < rect2.x + rect2.w &&
    // rect1.x + rect1.w > rect2.x &&
    // rect1.y < rect2.y + rect2.h &&
    // rect1.h + rect1.y > rect2.y
    if o.1.x < t.1.x + t.0.x
        && o.1.x + o.0.x > t.1.x
        && o.1.y < t.1.y + t.0.y
        && o.1.y + o.0.y > t.1.y
    {
        return true;
    }

    false
}

fn spawn_block(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    const MAX: f32 = 200.;
    let x = rng.gen_range(-MAX, MAX);
    let y = rng.gen_range(-MAX, MAX);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.2),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                scale: Vec3::new(25., 25., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider);
}

fn handle_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut snake_query: Query<(&Snake, &Transform)>,
    mut query: Query<(Entity, &Collider, &Transform)>,
) {
    let (_, snake_transform) = snake_query.single_mut();
    let snake_size = snake_transform.scale.truncate();
    let snake_pos = snake_transform.translation.truncate();

    for (entity, _, trans) in query.iter_mut() {
        if collides(
            (snake_size, snake_pos),
            (trans.scale.truncate(), trans.translation.truncate()),
        ) {
            commands.entity(entity).despawn();
            score.0 += 1;
            spawn_block(&mut commands);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.9, 0.9)))
        .insert_resource(Score(0))
        .add_startup_system(setup)
        .add_system(Snake::handle_keyboard.system())
        .add_system(change_title.system())
        .add_system(handle_collision.system())
        .add_system(score_system.system())
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
