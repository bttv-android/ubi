#![deny(clippy::all)]
#![deny(
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::filetype_is_file,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::inline_asm_x86_att_syntax,
    clippy::integer_division,
    clippy::mem_forget,
    clippy::multiple_inherent_impl,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::string_to_string,
    clippy::todo,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix
)]
#![warn(clippy::pedantic)]

#[macro_use]
extern crate tracing;

mod args;
mod err;
mod logging;

use aar::prepare_mock_aar;
use args::Args;
use err::ApplicationError;

fn main() -> miette::Result<()> {
    logging::setup();
    let args = args::parse();
    if let Err(err) = run(args) {
        return Err(miette::Report::new(err));
    }
    Ok(())
}

fn run(args: Args) -> Result<(), ApplicationError> {
    let _mocks_dir_path = prepare_mock_aar(args.mock_aar, &args.dx_path, &args.baksmali_path)?;
    // TODO
    Ok(())
}
