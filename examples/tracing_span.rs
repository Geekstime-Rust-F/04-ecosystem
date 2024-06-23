use tracing::{info, span, Level};

fn main() {
    tracing_subscriber::fmt::init();

    let main_span = span!(Level::INFO, "main_span");
    let _enter = main_span.enter();

    info!("Application started!");

    some_function();
}

fn some_function() {
    let span = span!(Level::INFO, "some_function_span");
    let _enter = span.enter();

    info!("Inside some_function!");

    std::thread::sleep(std::time::Duration::from_secs(1));

    info!("Exiting some_function!");
}
