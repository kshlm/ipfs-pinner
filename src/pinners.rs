pub mod infura;
pub mod pinata;

use std::path::Path;

use anyhow::Result;
use strum_macros::{AsRefStr, Display, EnumString, EnumVariantNames};

#[async_trait::async_trait]
pub trait Pinner {
    async fn pin_path(&self, path: &Path) -> Result<String>;
    async fn pin_hash(&self, hash: &str) -> Result<()>;
}

#[derive(Debug, AsRefStr, Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum Pinners {
    Infura,
    Pinata,
}
