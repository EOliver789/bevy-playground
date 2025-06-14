use bevy::prelude::*;


fn setup(
) {

}

fn hello_world(
) {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("Hello, world {}!", n);
        }
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, hello_world)
        .run();
}
