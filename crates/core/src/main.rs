use std::path;

use loader::load;

mod loader;

fn main() {
    // TODO: path
    let config_path = path::Path::new("./schema/example/.greqlint");

    let schema = load(config_path.to_str().unwrap());
    println!("{:?}", schema);
}
