use confy;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenConfig {
    pub secret: String,
    pub digits: usize,
    pub timestep: u64,
    pub format: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct TOTEMConfig {
    pub tokens: HashMap<String, TokenConfig>
}

impl ::std::default::Default for TOTEMConfig {
    fn default() -> Self { Self { tokens: HashMap::new() } }
}

pub fn get_config() -> Result<TOTEMConfig> {
    let cfg: TOTEMConfig = confy::load("TOTEM")?;
    Ok(cfg)
}

pub fn store_config(cfg: TOTEMConfig) -> Result<()> {
    confy::store("TOTEM", cfg)?;
    Ok(())
}
