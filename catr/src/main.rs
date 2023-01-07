fn main() {
    if let Err(e) = crate::run() {
        eprintln!("{}",e);
        std::process::exit(1);
    }
}
