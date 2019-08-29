extern crate clap;

use clap::{Arg, App, SubCommand};
use std::process::exit;

fn main() {
    let matches = App::new("kvs")
        .version("0.1.0")
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(_match)) => {
            eprintln!("unimplemented");
            exit(1);
        },
        ("get", Some(_match)) => {
            eprintln!("unimplemented");
            exit(1);
        },
        _ => unreachable!()
    }
}