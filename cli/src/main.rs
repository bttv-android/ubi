#[macro_use]
extern crate tracing;

mod args;
mod logging;

use aar::prepare_mock_aar;

fn main() {
    logging::setup();
    let args = args::parse();

    let _ = prepare_mock_aar(args.mock_aar, &args.dx_path, &args.baksmali_path);
}
