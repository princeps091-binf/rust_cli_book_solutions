use clap::{arg, Command, command, ArgAction};


fn main() {
    let matches = Command::new("echor_vipin")
    .version("0.0.1")
    .author("Vipin Kumar <princeps091@gmail.com>")
    .about("my implementation of echo in Rust")
    .arg(arg!([input] "the string to print").required(true))
    .get_matches();


    println!(
        "your string was: {:?}",
        matches.get_one::<String>("input").expect("required")
    );
}
