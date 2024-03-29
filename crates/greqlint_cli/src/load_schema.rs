use greqlint_schema::schema::Schema;
use serde_json::from_str;

pub fn load(path: &str) -> Result<Schema, Box<dyn std::error::Error>> {
    let res = from_str::<Schema>(&std::fs::read_to_string(path)?)?;
    Ok(res)
}
