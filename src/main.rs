use std::path::PathBuf;

fn main() {
    let arguments::Arguments {
        options, orphans, ..
    } = arguments::parse(std::env::args()).expect("failed to parse arguments");
    let source = PathBuf::from(orphans.get(0).expect("a source should be given"));
    let destination = PathBuf::from(orphans.get(1).expect("a destination should be given"));
    woff::version2::convert(
        source,
        destination,
        options.get::<String>("metadata"),
        options.get::<usize>("quality"),
        options.get::<bool>("transform"),
    )
    .expect("failed to convert");
}
