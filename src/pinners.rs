pub mod infura;
pub mod pinata;

use std::path::Path;

use anyhow::Result;
use strum_macros::{AsRefStr, Display, EnumString, EnumVariantNames};

#[async_trait::async_trait]
pub trait Pinner {
    async fn pin(&self, path: &Path) -> Result<String>;
}

#[derive(Debug, AsRefStr, Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum Pinners {
    Infura,
    Pinata,
}
