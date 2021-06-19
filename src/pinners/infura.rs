use std::path::Path;

use anyhow::{anyhow, Result};
use clap::Clap;
use ipfs_api::{IpfsClient, TryFromUri};

use crate::pinners::Pinner;

#[derive(Clap, Debug, Default)]
pub struct Infura {}

impl Infura {
    const ENDPOINT: &'static str = "https://ipfs.infura.io:5001";

    fn client() -> Result<IpfsClient> {
        IpfsClient::from_str(Self::ENDPOINT).map_err(|e| anyhow!(e))
    }
}

#[async_trait::async_trait]
impl Pinner for Infura {
    async fn pin_path(&self, path: &Path) -> Result<String> {
        match Self::client()?.add_path(path).await {
            Ok(res) => {
                let basename = path
                    .file_name()
                    .unwrap_or_else(|| unreachable!())
                    .to_str()
                    .unwrap_or_else(|| unreachable!());

                match res.iter().find(|r| r.name.as_str() == basename) {
                    Some(r) => Ok(r.hash.clone()),
                    None => Err(anyhow!("could not find root hash")),
                }
            }
            Err(err) => Err(anyhow!(err)),
        }
    }

    async fn pin_hash(&self, hash: &str) -> Result<()> {
        match Self::client()?.pin_add(hash, true).await {
            Ok(_) => Ok(()),
            Err(err) => Err(anyhow!(err)),
        }
    }
}
