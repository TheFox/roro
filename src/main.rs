
extern crate clap;
use clap::App;
// use clap::Arg;
// use clap::ArgMatches;
use std::env::args;

// const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_NAME: &'static str = "roro";
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

/// Main
fn main() {
    println!("-> start");

    let args: Vec<String> = args().collect();
    println!("-> args: '{:?}'", args);

    // Vars Sub Command
    let vars_subcmd = App::new("vars")
        .about("Print variables.");

    // Compile Sub Command
    let compile_subcmd = App::new("compile")
        .about("Compile");

    // Main App
    let app = App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHORS)
        .about(APP_HOMEPAGE)
        .usage("roro [OPTIONS] [SUBCOMMAND] [SUBCOMMAND_OPTIONS]")
        .subcommand(vars_subcmd)
        .subcommand(compile_subcmd);

    // Get Argument matches.
    let matches = app.get_matches();
    // println!("-> matches '{:?}'", matches);

    match matches.subcommand() {
        ("vars", _) => {
            println!("-> cmd: vars");
            println!("APP_NAME '{}'", APP_NAME);
            println!("APP_VERSION '{}'", APP_VERSION);

            return;
        },
        ("compile", Some(_compile_matches)) => {
            println!("-> cmd: compile");
        },
        _ => {
            println!("No command.");
        },
    }

    println!("-> end");
}
