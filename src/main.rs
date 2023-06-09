use std::path::PathBuf;

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let source = PathBuf::from(
        arguments
            .get::<String>("source")
            .expect("--source should be given"),
    );
    let destination = PathBuf::from(
        arguments
            .get::<String>("destination")
            .expect("--destination should be given"),
    );
    let metadata = arguments
        .get::<String>("metadata")
        .unwrap_or_else(|| "".into());
    let quality = arguments.get::<usize>("quality").unwrap_or(8);
    let transform = arguments.get::<bool>("transform").unwrap_or(true);
    woff::version2::convert(source, destination, metadata, quality, transform)
        .expect("failed to convert");
}
