use loader::load;
use std::path;

mod linter;
mod loader;
mod normalizer;

fn main() {
    // TODO: path
    let config_path = path::Path::new("./schema/example/.greqlint");

    let schema = load(config_path.to_str().unwrap());
    println!("{:?}", schema);
}
