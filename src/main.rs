#![feature(ascii_char)]

use clap::{command, value_parser, Arg, Command};

mod generate;
mod format;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .subcommands([
            Command::new("pair").about("Generate username-password pair")
            .args([
                Arg::new("username length").short('u')
                    .value_parser(value_parser!(usize))
                    .default_value("5"),
                Arg::new("password length").short('p')
                    .value_parser(value_parser!(usize))
                    .default_value("24"),
            ]),
            Command::new("pass").about("Generate single password")
            .args([
                Arg::new("password length").short('p')
                    .value_parser(value_parser!(usize))
                    .default_value("24"),
            ]),
        ]).get_matches();

    let rng = rand::thread_rng();
    
    match matches.subcommand() {
        Some(("pair", sub)) => {
            print!("{}", format::pair(generate::pair(rng, (
                *sub.get_one::<usize>("username length").unwrap(),
                *sub.get_one::<usize>("password length").unwrap(),
            ))))
        },
        Some(("pass", sub)) => {
            print!("{}", format::pass(generate::pass(rng,
                *sub.get_one::<usize>("password length").unwrap(),
            )))
        },
        _ => unreachable!(),
    }
}
