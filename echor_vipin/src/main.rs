use clap::{Arg, command, ArgAction};
fn main() {
    let matches = command!()
    .arg(
        Arg::new("input")
//                .short('i')
//                .long("input")
                .help("Input string from user")
                .action(ArgAction::Set)
                .required(true),
    )
    .arg(
        Arg::new("newline")
            .short('n')
            .long("newline")
            .help("Flag to add trailing newline to input")
            .action(ArgAction::SetTrue)
    )
    .get_matches();

    let text = matches.get_one::<String>("input").unwrap();
    let omit_newline = matches.get_one("newline").unwrap();

    let ending = if *omit_newline { "\n" } else { "" };
    
    print!("{}{}", text, ending);
}
