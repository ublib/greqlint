use schemars::schema::Schema;
use serde_json::from_str;

pub mod schema;

pub fn load(path: &str) -> Result<Schema, Box<dyn std::error::Error>> {
    let res = from_str::<Schema>(&std::fs::read_to_string(path)?)?;
    Ok(res)
}
