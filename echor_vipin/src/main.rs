use clap::Command;
fn main() {
    let _matches = Command::new("echor_vipin")
    .version("0.1.0")
    .author("Vipin Kumar <princeps091@gmail.com>")
    .about("My implementation of echo in Rust")
    .get_matches();

}
