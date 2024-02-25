extern crate ansi_colors;
use ansi_colors::*;

use clap::Parser;
use greqlint_core::linter::title::lint_title;

use crate::load_schema::load;
use std::{path, process::exit};

mod args;
mod load_schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: normalize loading
    let title = std::env::var("CI_MERGE_REQUEST_TITLE").unwrap();

    let args = args::Args::parse();

    let config_path = path::Path::new(&args.config_path);
    let schema = load(config_path.to_str().unwrap())?;

    match lint_title(&schema, &title) {
        Ok(_) => {}
        Err(e) => {
            let mut message = ColouredStr::new("Lint Error");
            message.red();
            println!("{message}: {e}");
            exit(1);
        }
    };
    Ok(())
}
