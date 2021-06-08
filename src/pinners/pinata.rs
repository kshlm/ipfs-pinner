use std::path::Path;

use anyhow::Result;
use clap::Clap;

use crate::pinners::{Pinner, Pinners};

#[derive(Clap, Debug)]
pub struct Pinata {
    /// Pinata API key
    #[clap(
        long = "pinata-api-key",
        env = "PINATA_API_KEY",
        hide_env_values = true,
        required_if_eq("pinner", Pinners::Pinata.as_ref()),
    )]
    api_key: Option<String>,
    /// Pinata API secret key
    #[clap(
        long = "pinata-secret",
        env = "PINATA_SECRET",
        hide_env_values = true,
        required_if_eq("pinner", Pinners::Pinata.as_ref()),
    )]
    secret: Option<String>,
}

#[async_trait::async_trait]
impl Pinner for Pinata {
   async fn pin(&self, _: &Path) -> Result<String> {
       todo!()
    }
}


