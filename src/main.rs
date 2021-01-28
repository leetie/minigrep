use minigrep::Config;
use std::process;

fn main() {
    let config = Config::new(minigrep::init_clap()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
