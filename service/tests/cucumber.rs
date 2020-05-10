use cucumber_rust::cucumber;

mod steps;
mod world;

pub use world::World;

fn setup() {
    let _ = tracing_subscriber::fmt::try_init();
}

cucumber! {
    features: "./tests/features",
    world: World,
    steps: &[
        steps::health::steps,
        steps::users::steps
    ],
    setup: setup
}
