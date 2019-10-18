use clap::{App, Arg, SubCommand};
use std::error::Error;
use std::process;

use blake;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("blake")
        .version("0.1.0")
        .author("Jonathan M. Lange <jml@mumak.net>")
        .about("Situated blogging platform")
        .subcommand(SubCommand::with_name("new"))
        .subcommand(SubCommand::with_name("edit"))
        .subcommand(
            SubCommand::with_name("build")
                .arg(
                    Arg::with_name("--rebuild").help("Rebuild everything, even if it's up-to-date"),
                )
                .arg(
                    Arg::with_name("--posts-only")
                        .help("Only build posts, don't build the indexes."),
                ),
        );
    let matches = app.get_matches();
    match matches.subcommand_name() {
        Some("new") => blake::new_post()?,
        Some("edit") => blake::edit_post()?,
        Some("build") => blake::build()?,
        Some(_) | None => {
            println!("Invalid subcommand given.");
            process::exit(2);
        }
    }
    Ok(())
}
