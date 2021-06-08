use anyhow::Result;
use clap::Clap;

use crate::dnslinkers::{DnsLinker, DnsLinkers};

#[derive(Clap, Debug)]
pub struct Cloudflare {
    /// Cloudflare API token
    #[clap(
        long = "cf-api-token",
        env = "CF_API_TOKEN",
        hide_env_values = true,
        required_if_eq("dns", DnsLinkers::Cloudflare.as_ref()),
    )]
    api_token: Option<String>,
    /// Cloudflare DNS zone
    #[clap(
        long = "cf-zone",
        env = "CF_ZONE",
        hide_env_values = true,
        required_if_eq("dns", DnsLinkers::Cloudflare.as_ref()),
    )]
    dns_zone: Option<String>,
}

#[async_trait::async_trait]
impl DnsLinker for Cloudflare {
    async fn link(&self, _hash: &str) -> Result<()> {
        unimplemented!();
    }
}
