#![feature(ascii_char)]

use clap::{command, value_parser, Arg, Command};

mod generate;
mod format;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .subcommands([
            Command::new("userpass").about("Generate username-password pair")
            .args([
                Arg::new("username length").short('u')
                    .value_parser(value_parser!(usize))
                    .default_value("5"),
                Arg::new("password length").short('p')
                    .value_parser(value_parser!(usize))
                    .default_value("24"),
            ]),
        ]).get_matches();

    let rng = rand::thread_rng();
    
    match matches.subcommand() {
        Some(("userpass", sub)) => {
            print!("{}", format::userpass(generate::userpass(rng, (
                *sub.get_one::<usize>("username length").unwrap(),
                *sub.get_one::<usize>("password length").unwrap(),
            ))))
        },
        _ => unreachable!(),
    }
}
