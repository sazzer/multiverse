use cucumber_rust::cucumber;

mod health;
mod users;
pub mod world;

pub use world::World;

fn setup() {
    let _ = tracing_subscriber::fmt::try_init();
}

cucumber! {
    features: "./tests/features",
    world: World,
    steps: &[
        health::steps,
        users::steps
    ],
    setup: setup
}
