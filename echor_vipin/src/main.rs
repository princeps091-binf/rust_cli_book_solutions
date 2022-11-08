use clap::{Arg, command, ArgAction};
fn main() {
    let matches = command!()
    .arg(
        Arg::new("input")
//                .short('i')
//                .long("input")
                .help("Input string from user")
                .action(ArgAction::Append)
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

    let text: _  = matches.get_many::<String>("input")
    .expect("input string required")
    .map(|v| v.as_str())
    .collect::<Vec<&str>>().join(" ");

    let omit_newline = matches.get_one("newline").unwrap();

    let ending = if *omit_newline { "\n" } else { "" };
    
    print!("{}{}", text, ending);
}
