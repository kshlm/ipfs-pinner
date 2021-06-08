pub mod cloudflare;

use anyhow::Result;
use strum_macros::{AsRefStr, Display, EnumString, EnumVariantNames};

#[async_trait::async_trait]
pub trait DnsLinker {
    async fn link(&self, hash: &str) -> Result<()>;
}

#[derive(Debug, AsRefStr, Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum DnsLinkers {
    Cloudflare,
}

