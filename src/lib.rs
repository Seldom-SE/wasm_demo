use bevy::{prelude::*, render::camera::Camera};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use rand::prelude::*;
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
        .add_event::<CheckPlayerWinEvent>()
        .add_event::<AIEvent>()
        .add_event::<CheckAiWinEvent>()
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
                .with_system(
                    consume_click_event
                        .system()
                        .label("consume_click_event")
                        .after("detect_click"),
                )
                .with_system(
                    check_player_win
                        .system()
                        .label("check_player_win")
                        .after("consume_click_event"),
                )
                .with_system(
                    ai_click
                        .system()
                        .label("ai_click")
                        .after("check_player_win"),
                )
                .with_system(
                    check_ai_win
                        .system()
                        .label("check_ai_win")
                        .after("ai_click"),
                ),
        )
        .run();
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum GameState {
    Load,
    Game,
    End,
}

#[derive(AssetCollection, Clone, Default)]
struct Textures {
    #[asset(path = "textures/board.png")]
    board: Handle<Texture>,
    #[asset(path = "textures/x.png")]
    x: Handle<Texture>,
    #[asset(path = "textures/o.png")]
    o: Handle<Texture>,
    #[asset(path = "textures/unclicked.png")]
    unclicked: Handle<Texture>,
    #[asset(path = "textures/x_win.png")]
    x_win: Handle<Texture>,
    #[asset(path = "textures/o_win.png")]
    o_win: Handle<Texture>,
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
    let text = materials.add(textures.unclicked.clone().into());
    let text1 = materials.add(textures.x.clone().into());
    let text2 = materials.add(textures.o.clone().into());
    let text3 = materials.add(textures.x_win.clone().into());
    let text4 = materials.add(textures.o_win.clone().into());

    commands.insert_resource(Materials {
        matUnclicked: text.clone(),
        matX: text1.clone(),
        matO: text2.clone(),
        matXWin: text3.clone(),
        matOWin: text4.clone(),
    });
    // TEMP for demonstration purposes
    let middle_middle = commands
        .spawn_bundle(SpriteBundle {
            material: text.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..SpriteBundle::default()
        })
        .insert(Clickable {
            half_extents: Vec2::splat(100.),
        })
        .id();
    let right_middle = commands
        .spawn_bundle(SpriteBundle {
            material: text.clone(),
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            ..SpriteBundle::default()
        })
        .insert(Clickable {
            half_extents: Vec2::splat(100.),
        })
        .id();
    let left_middle = commands
        .spawn_bundle(SpriteBundle {
            material: text.clone(),
            transform: Transform::from_xyz(-200.0, 0.0, 0.0),
            ..SpriteBundle::default()
        })
        .insert(Clickable {
            half_extents: Vec2::splat(100.),
        })
        .id();
    let left_bottom = commands
        .spawn_bundle(SpriteBundle {
            material: text.clone(),
            transform: Transform::from_xyz(-200.0, -200.0, 0.0),
            ..SpriteBundle::default()
        })
        .insert(Clickable {
            half_extents: Vec2::splat(100.),
        })
        .id();
    let right_bottom = commands
        .spawn_bundle(SpriteBundle {
            material: text.clone(),
            transform: Transform::from_xyz(200.0, -200.0, 0.0),
            ..SpriteBundle::default()
        })
        .insert(Clickable {
            half_extents: Vec2::splat(100.),
        })
        .id();
    let middle_bottom = commands
        .spawn_bundle(SpriteBundle {
            material: text.clone(),
            transform: Transform::from_xyz(0.0, -200.0, 0.0),
            ..SpriteBundle::default()
        })
        .insert(Clickable {
            half_extents: Vec2::splat(100.),
        })
        .id();
    let middle_top = commands
        .spawn_bundle(SpriteBundle {
            material: text.clone(),
            transform: Transform::from_xyz(0.0, 200.0, 0.0),
            ..SpriteBundle::default()
        })
        .insert(Clickable {
            half_extents: Vec2::splat(100.),
        })
        .id();
    let right_top = commands
        .spawn_bundle(SpriteBundle {
            material: text.clone(),
            transform: Transform::from_xyz(200.0, 200.0, 0.0),
            ..SpriteBundle::default()
        })
        .insert(Clickable {
            half_extents: Vec2::splat(100.),
        })
        .id();
    let left_top = commands
        .spawn_bundle(SpriteBundle {
            material: text.clone(),
            transform: Transform::from_xyz(-200.0, 200.0, 0.0),
            ..SpriteBundle::default()
        })
        .insert(Clickable {
            half_extents: Vec2::splat(100.),
        })
        .id();

    let clickableEntities = ClickableEntities {
        sections: [
            [left_top, middle_top, right_top],
            [left_middle, middle_middle, right_middle],
            [left_bottom, middle_bottom, right_bottom],
        ],
    };
    commands.insert_resource(clickableEntities);
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

struct Materials {
    matUnclicked: Handle<ColorMaterial>,
    matX: Handle<ColorMaterial>,
    matO: Handle<ColorMaterial>,
    matXWin: Handle<ColorMaterial>,
    matOWin: Handle<ColorMaterial>,
}

struct CheckPlayerWinEvent;

fn consume_click_event(
    mut check_player_win_event: EventWriter<CheckPlayerWinEvent>,
    mut click_events: EventReader<ClickEvent>,
    mut material: Query<&mut Handle<ColorMaterial>>,
    matCompare: Res<Materials>,
) {
    click_events.iter().for_each(|click_event| {
        if *material.get_mut(click_event.0).unwrap() == matCompare.matUnclicked {
            *material.get_mut(click_event.0).unwrap() = matCompare.matX.clone();
            check_player_win_event.send(CheckPlayerWinEvent);
        }
    });
}

fn check_win(
    current_player: Handle<ColorMaterial>,
    material: &Query<&Handle<ColorMaterial>>,
    clickable_entities: &Res<ClickableEntities>,
) -> bool {
    let mut win = false;

    for row in 0..3 {
        let mut row_win = true;
        for col in 0..3 {
            row_win &=
                *material.get(clickable_entities.sections[row][col]).unwrap() == current_player;
        }
        win |= row_win;
    }

    for col in 0..3 {
        let mut col_win = true;
        for row in 0..3 {
            col_win &=
                *material.get(clickable_entities.sections[row][col]).unwrap() == current_player;
        }
        win |= col_win;
    }

    let mut diag_win_1 = true;
    let mut diag_win_2 = true;

    for diag in 0..3 {
        diag_win_1 &= *material
            .get(clickable_entities.sections[diag][diag])
            .unwrap()
            == current_player;
        diag_win_2 &= *material
            .get(clickable_entities.sections[2 - diag][diag])
            .unwrap()
            == current_player;
    }

    win |= diag_win_1;
    win |= diag_win_2;

    win
}

struct AIEvent;

fn check_player_win(
    mut commands: Commands,
    mut app_state: ResMut<State<GameState>>,
    mut ai_events: EventWriter<AIEvent>,
    mut check_player_win_events: EventReader<CheckPlayerWinEvent>,
    clickable_entities: Res<ClickableEntities>,
    material: Query<&Handle<ColorMaterial>>,
    matCompare: Res<Materials>,
) {
    check_player_win_events.iter().for_each(|_| {
        if check_win(matCompare.matX.clone(), &material, &clickable_entities) {
            commands.spawn_bundle(SpriteBundle {
                material: matCompare.matXWin.clone(),
                transform: Transform::from_xyz(0., 0., 1.),
                ..SpriteBundle::default()
            });

            app_state.set(GameState::End).unwrap();
        } else {
            ai_events.send(AIEvent);
        }
    });
}

struct ClickableEntities {
    //[type;size]
    sections: [[Entity; 3]; 3],
}

struct CheckAiWinEvent;

fn ai_click(
    mut ai_events: EventReader<AIEvent>,
    mut check_ai_win_events: EventWriter<CheckAiWinEvent>,
    clickable_entities: Res<ClickableEntities>,
    mut material: Query<&mut Handle<ColorMaterial>>,
    matCompare: Res<Materials>,
) {
    ai_events.iter().for_each(|ai_event| {
        loop {
            let x = rand::thread_rng().gen_range(0..3);
            let y = rand::thread_rng().gen_range(0..3);
            if *material.get_mut(clickable_entities.sections[y][x]).unwrap()
                == matCompare.matUnclicked
            {
                *material.get_mut(clickable_entities.sections[y][x]).unwrap() =
                    matCompare.matO.clone();
                break;
            }
        }

        check_ai_win_events.send(CheckAiWinEvent);
    });
}

fn check_ai_win(
    mut commands: Commands,
    mut app_state: ResMut<State<GameState>>,
    mut check_ai_win_events: EventReader<CheckAiWinEvent>,
    clickable_entities: Res<ClickableEntities>,
    material: Query<&Handle<ColorMaterial>>,
    matCompare: Res<Materials>,
) {
    check_ai_win_events.iter().for_each(|_| {
        if check_win(matCompare.matO.clone(), &material, &clickable_entities) {
            commands.spawn_bundle(SpriteBundle {
                material: matCompare.matOWin.clone(),
                transform: Transform::from_xyz(0., 0., 1.),
                ..SpriteBundle::default()
            });

            app_state.set(GameState::End).unwrap();
        }
    });
}
