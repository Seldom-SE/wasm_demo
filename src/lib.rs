use bevy::prelude::*;
use wasm_bindgen::prelude::*;

struct Person;
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("What up".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Test Name".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system());

    app.run();
}

fn hello_world() {
    println!("hello world!");
}
