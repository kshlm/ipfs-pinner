pub mod pinners;
pub mod dnslinkers;

use std::{convert::AsRef, path::PathBuf, string::ToString};

use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, Clap};
use strum::VariantNames;

use crate::pinners::{Pinner, Pinners, pinata::Pinata, infura::Infura};
use crate::dnslinkers::{DnsLinker, DnsLinkers, cloudflare::Cloudflare};

/// Upload a path to a IPFS pinning service and update a DNSLINK record of choice
#[derive(Clap, Debug)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!()
)]
struct Config {
    /// Path to be pinned
    #[clap(parse(from_os_str))]
    path: PathBuf,
    /// Pinning service to be used
    #[clap(
        short,
        long,
        default_value = Pinners::Infura.as_ref(),
        possible_values = &Pinners::VARIANTS,
    )]
    pinner: Pinners,
    /// DNS provider to update DNSLINK record
    #[clap(
        short,
        long,
        possible_values = &DnsLinkers::VARIANTS,
        requires = "dnslink-record",
    )]
    dns: Option<DnsLinkers>,

    /// DNS record to be set with DNSLINK value
    #[clap(long, env, hide_env_values = true)]
    dnslink_record: Option<String>,

    #[clap(flatten)]
    cloudflare: Cloudflare,

    #[clap(flatten)]
    infura: Infura,

    #[clap(flatten)]
    pinata: Pinata,

}


#[async_std::main]
async fn main() -> Result<()> {
    let conf: Config = Config::parse();

    let pinner: &dyn Pinner = match conf.pinner {
        Pinners::Infura => &conf.infura as &dyn Pinner,
        Pinners::Pinata => &conf.pinata as &dyn Pinner,
    };
    let hash = pinner.pin(conf.path.as_path()).await?;

    println!("Pinned {} with hash {}", conf.path.display(), hash);

    if let Some(dns) = &conf.dns {
        let record = conf.dnslink_record.unwrap_or_else(|| unreachable!());
        let linker: &dyn DnsLinker = match dns {
            DnsLinkers::Cloudflare => &conf.cloudflare as &dyn DnsLinker,
        };
        let _ = linker.link(&hash, record.as_ref()).await?;
        println!("Update dnslink record for {} with dnslink=/ipfs/{}", record, hash);
    }

    Ok(())
}
