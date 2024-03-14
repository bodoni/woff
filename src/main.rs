use std::path::PathBuf;

fn main() {
    let arguments::Arguments {
        options, orphans, ..
    } = arguments::parse(std::env::args()).expect("failed to parse arguments");
    #[allow(clippy::get_first)]
    let source = match orphans.get(0) {
        Some(value) => PathBuf::from(value),
        _ => {
            eprintln!("Error: a source should be given.");
            std::process::exit(1);
        }
    };
    let destination = match orphans.get(1) {
        Some(value) => PathBuf::from(value),
        _ => {
            eprintln!("Error: a destination should be given.");
            std::process::exit(1);
        }
    };
    woff::version2::convert(
        source,
        destination,
        options.get::<String>("metadata"),
        options.get::<usize>("quality"),
        options.get::<bool>("transform"),
    )
    .expect("failed to convert");
}
