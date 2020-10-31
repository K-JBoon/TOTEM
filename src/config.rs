use confy;
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
struct TOTPConfig {
    secret: String,
    digits: usize,
    timestep: u64,
    format: String
}

#[derive(Serialize, Deserialize)]
pub struct TOTEMConfig {
    tokens: Vec<TOTPConfig>
}

impl ::std::default::Default for TOTEMConfig {
    fn default() -> Self { Self { tokens: vec![] } }
}

pub fn get_config() -> Result<TOTEMConfig> {
    let cfg: TOTEMConfig = confy::load("TOTEM")?;
    Ok(cfg)
}
