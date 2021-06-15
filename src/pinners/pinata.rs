use std::path::Path;

use anyhow::{anyhow, Result};
use clap::Clap;
use pinata_sdk::{PinByFile, PinataApi};

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
    async fn pin(&self, path: &Path) -> Result<String> {
        match (self.api_key.as_ref(), self.secret.as_ref()) {
            (Some(api_key), Some(secret)) => {
                let pinata = PinataApi::new(api_key, secret).map_err(|e| anyhow!(e))?;
                match pinata
                    .pin_file(PinByFile::new(
                        path.to_str().unwrap_or_else(|| unreachable!()),
                    ))
                    .await
                {
                    Ok(res) => Ok(res.ipfs_hash),
                    Err(err) => Err(anyhow!(err)),
                }
            }
            _ => unreachable!(),
        }
    }
}
