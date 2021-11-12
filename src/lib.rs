use bevy::{prelude::*, render::camera::Camera};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();

    AssetLoader::new(GameState::Load, GameState::Game)
        .with_collection::<Textures>()
        .build(&mut app);

    app.insert_resource(WindowDescriptor {
        title: "Tic-Tac-Toe".to_string(),
        vsync: false,
        ..WindowDescriptor::default()
    })
    .insert_resource(ClearColor(Color::WHITE))
    .add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.init_resource::<CursorWorldPos>()
        .add_state(GameState::Load)
        .add_event::<ClickEvent>()
        .add_system_set(SystemSet::on_enter(GameState::Game).with_system(init.system()))
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(
                    update_cursor_world_pos
                        .system()
                        .label("update_cursor_world_pos"),
                )
                .with_system(
                    detect_click
                        .system()
                        .label("detect_click")
                        .after("update_cursor_world_pos"),
                )
                // TEMP for demonstration purposes
                .with_system(
                    consume_click_event
                        .system()
                        .label("consume_click_event")
                        .after("detect_click"),
                ),
        )
        .run();
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum GameState {
    Load,
    Game,
}

#[derive(AssetCollection, Clone, Default)]
struct Textures {
    #[asset(path = "textures/board.png")]
    board: Handle<Texture>,
    #[asset(path = "textures/x.png")]
    x: Handle<Texture>,
    #[asset(path = "textures/o.png")]
    o: Handle<Texture>,
}

fn init(
    mut commands: Commands,
    textures: Res<Textures>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(textures.board.clone().into()),
        ..SpriteBundle::default()
    });

    // TEMP for demonstration purposes
    commands
        .spawn()
        .insert(Clickable {
            half_extents: Vec2::splat(100.),
        })
        .insert(Transform::default());
}

#[derive(Default)]
struct CursorWorldPos(Vec2);

fn cursor_world_pos(window: &Window, camera_transform: &Transform) -> Option<Vec2> {
    window.cursor_position().map(|cursor_pos| {
        (camera_transform.compute_matrix()
            * (cursor_pos - Vec2::new(window.width() as f32, window.height() as f32) / 2.)
                .extend(0.)
                .extend(1.))
        .truncate()
        .truncate()
    })
}

fn update_cursor_world_pos(
    camera_query: Query<&Transform, With<Camera>>,
    windows: Res<Windows>,
    mut pos: ResMut<CursorWorldPos>,
) {
    if let Some(new_pos) = cursor_world_pos(
        windows.get_primary().unwrap(),
        camera_query.single().unwrap(),
    ) {
        pos.0 = new_pos;
    }
}

struct Clickable {
    half_extents: Vec2,
}

struct ClickEvent(Entity);

fn detect_click(
    mut click_events: EventWriter<ClickEvent>,
    clickables: Query<(Entity, &Clickable, &Transform)>,
    cursor_world_pos: Res<CursorWorldPos>,
    mouse: Res<Input<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        clickables.for_each(|(entity, clickable, transform)| {
            if clickable
                .half_extents
                .cmpgt((transform.translation.truncate() - cursor_world_pos.0).abs())
                .all()
            {
                click_events.send(ClickEvent(entity));
            }
        });
    }
}

// TEMP for demonstration purposes
fn consume_click_event(mut click_events: EventReader<ClickEvent>) {
    click_events.iter().for_each(|_| {
        println!("Clicked!");
    });
}
