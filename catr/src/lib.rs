use std::error::Error;
use clap::{Arg, command, ArgAction};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}


pub fn get_args() -> MyResult<Config> {

    let matches = command!()
    .arg(
        Arg::new("files")
            .short('f')
            .long("files")
            .help("Files to concatenate")
            .action(ArgAction::Append)
            .num_args(0..)
            .default_value("_")
        )
    .arg(
        Arg::new("number")
            .short('n')
            .long("number")
            .help("Number lines")
            .action(ArgAction::SetTrue)
            .conflicts_with("number_nonblank")
        )
    .arg(
        Arg::new("number_nonblank")
            .short('b')
            .long("number-nonblank")
            .help("Number nonblank lines")
            .action(ArgAction::SetTrue)
            .conflicts_with("number")
    )
    .get_matches();

    Ok(Config {
        files: matches.get_many::<String>("files")
        .unwrap_or_default()
        .map(|v| v.to_string())
        .collect::<Vec<String>>(),
        number_lines: *matches.get_one("number").unwrap(),
        number_nonblank_lines: *matches.get_one("number_nonblank").unwrap(),
        })
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()>{
    for filename in config.files {
        println!("{}",filename);
    }
    Ok(())
}