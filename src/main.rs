//! # Kibi

use kibi::{Config, Editor, Error};

/// Load the configuration, initialize the editor and run the program, optionally opening a file if
/// an argument is given.
///
/// # Errors
///
/// Any error that occur during the execution of the program will be returned by this function.
fn main() -> Result<(), Error> {
    let mut args = std::env::args();
    match (args.nth(1), args.nth(2)) {
        (_, Some(_)) => return Err(Error::TooManyArguments(args.len() - 1)),
        (Some(arg), _) if arg == "--version" =>
            println!(concat!("kibi, v", env!("CARGO_PKG_VERSION"))),
        (file_name, None) => Editor::new(Config::load()?)?.run(file_name)?,
    }
    Ok(())
}
