use std::path::Path;

use anyhow::{anyhow, Result};
use clap::Clap;
use ipfs_api::{IpfsClient, TryFromUri};

use crate::pinners::Pinner;

#[derive(Clap, Debug, Default)]
pub struct Infura {}

impl Infura {
    const ENDPOINT: &'static str = "https://ipfs.infura.io:5001";
}

#[async_trait::async_trait]
impl Pinner for Infura {
    async fn pin(&self, path: &Path) -> Result<String> {
        let ipfs = IpfsClient::from_str(Self::ENDPOINT)?;
        match ipfs.add_path(path).await {
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
}
