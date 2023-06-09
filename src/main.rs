use std::path::PathBuf;

fn main() {
    let arguments::Arguments {
        options, orphans, ..
    } = arguments::parse(std::env::args()).expect("failed to parse arguments");
    let source = PathBuf::from(orphans.get(0).expect("a source should be given"));
    let destination = PathBuf::from(orphans.get(1).expect("a destination should be given"));
    let metadata = options
        .get::<String>("metadata")
        .unwrap_or_else(|| "".into());
    let quality = options.get::<usize>("quality").unwrap_or(8);
    let transform = options.get::<bool>("transform").unwrap_or(true);
    woff::version2::convert(source, destination, metadata, quality, transform)
        .expect("failed to convert");
}
