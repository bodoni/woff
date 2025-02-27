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
    let source_extension = source.extension().and_then(|value| value.to_str());
    let destination_extension = destination.extension().and_then(|value| value.to_str());
    if source_extension
        .map(|value| value == "woff")
        .unwrap_or(false)
        || destination_extension
            .map(|value| value == "woff")
            .unwrap_or(false)
    {
        woff::version1::convert(source, destination).expect("failed to transform");
    } else if source_extension
        .map(|value| value == "woff2")
        .unwrap_or(false)
        || destination_extension
            .map(|value| value == "woff2")
            .unwrap_or(false)
    {
        woff::version2::convert(
            source,
            destination,
            options.get::<String>("metadata"),
            options.get::<usize>("quality"),
            options.get::<bool>("transform"),
        )
        .expect("failed to transform");
    } else {
        eprintln!("Error: one file should end with either .woff or .woff2.");
        std::process::exit(1);
    }
}
