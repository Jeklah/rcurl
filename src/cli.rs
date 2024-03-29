use clap::{Arg, ArgMatches, Command};

pub fn get_arguments() -> ArgMatches {
    Command::new("rcurl - curl implemented in rust.")
        .about("It helps to make HTTP methods")
        .version("1.0")
        .author("Jeklah")
        .arg(Arg::new("url").index(1).required(true))
        .arg(
            Arg::new("x-method")
                .help("HTTP method which you want to use.")
                .long("x-method")
                .short("X"),
        )
        .arg(
            Arg::new("data")
                .help("Payload you want to send with the request.")
                .long("data")
                .short("d"),
        )
        .arg(
            Arg::new("headers")
                .help("Request header.")
                .long("header")
                .short("H")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("verbose")
                .help("verbose mode.")
                .long("verbose")
                .short("V")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
}
