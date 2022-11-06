use clap::{Arg, command, ArgAction};


fn main() {
    let matches = command!()
    .arg(
        Arg::new("input")
                .short('i')
                .long("input")
                .action(ArgAction::Append),
    )
    .arg(
        Arg::new("newline")
            .short('n')
            .long("newline")
            .action(ArgAction::SetTrue)
    )
    .get_matches();

    let text = matches.get_one::<String>("input").unwrap();
    let omit_newline = matches.get_one("newline").unwrap();

    let mut ending = "";

    if *omit_newline {
        ending = "\n"; // This will not work
        }

    print!("{}{}", text, ending);
}
