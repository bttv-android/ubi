#[macro_use]
extern crate tracing;

mod args;
mod logging;

fn main() {
    logging::setup();
    let args = args::parse();
}
