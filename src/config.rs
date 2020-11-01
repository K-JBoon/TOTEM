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
pub struct TOTPGenConfig {
    pub tokens: HashMap<String, TokenConfig>
}

impl ::std::default::Default for TOTPGenConfig {
    fn default() -> Self { Self { tokens: HashMap::new() } }
}

pub fn get_config() -> Result<TOTPGenConfig> {
    let cfg: TOTPGenConfig = confy::load("totpgen")?;
    Ok(cfg)
}

pub fn store_config(cfg: TOTPGenConfig) -> Result<()> {
    confy::store("totpgen", cfg)?;
    Ok(())
}
