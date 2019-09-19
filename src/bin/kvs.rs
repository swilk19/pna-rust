extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("kvs")
        .version("0.1.0")
        .subcommand(
            SubCommand::with_name("set")
                .arg(
                    Arg::with_name("key")
                        .value_name("KEY")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("value")
                        .value_name("VALUE")
                        .required(true)
                        .takes_value(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("get").arg(
                Arg::with_name("key")
                    .value_name("KEY")
                    .required(true)
                    .takes_value(true)
                    .index(1),
            ),
        )
        .subcommand(
            SubCommand::with_name("rm").arg(
                Arg::with_name("key")
                    .value_name("KEY")
                    .required(true)
                    .takes_value(true)
                    .index(1),
            ),
        )
        .get_matches();

    let mut kvstore = kvs::KvStore::new();

    match matches.subcommand() {
        ("set", Some(matches)) => {
            let key = matches.value_of("key").unwrap_or_else(|| panic!());
            let value = matches.value_of("value").unwrap_or_else(|| panic!());
            kvstore.set(key.to_string(), value.to_string());
            println!("Set '{}' => '{}'.", key, value);
        }
        ("get", Some(matches)) => {
            let key = matches.value_of("key").unwrap_or_else(|| panic!());
            let value = kvstore.get(key.to_string());
            if value.is_some() {
                let found = value.unwrap();
                println!("Found '{}' => '{}'.", key, found);
            } else {
                println!("No value found for key '{}'.", key);
            }
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("key").unwrap_or_else(|| panic!());
            kvstore.remove(key.to_string());
        }
        _ => unreachable!(),
    }
}
