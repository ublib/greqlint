#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! serde = "*"
//! serde_json = "*"
//! schemars = "*"
//! greqlint_schema = { path = "../../crates/greqlint_schema" }
//! ```

use greqlint_schema::schema::Schema;

use schemars::schema_for;
use serde_json::to_writer_pretty;

use std::path;

const SCHEMA_PATH: &str = "./crates/greqlint_schema/schema.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate()?;
    Ok(())
}

fn generate() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = path::Path::new(SCHEMA_PATH);
    let schema = schema_for!(Schema);
    to_writer_pretty(std::fs::File::create(config_path)?, &schema).unwrap();
    Ok(())
}
