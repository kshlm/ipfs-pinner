use anyhow::{bail, Result};
use clap::Clap;
use cloudflare::{
    endpoints::{
        dns::{
            CreateDnsRecord, CreateDnsRecordParams, DnsContent, DnsRecord, ListDnsRecords,
            ListDnsRecordsParams, UpdateDnsRecord, UpdateDnsRecordParams,
        },
        zone::{ListZones, ListZonesParams},
    },
    framework::{
        async_api::{ApiClient, Client},
        auth::Credentials,
        Environment, HttpApiClientConfig,
    },
};

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
    async fn link(&self, hash: &str, record: &str) -> Result<()> {
        if let (Some(api_token), Some(dns_zone)) = (self.api_token.as_ref(), self.dns_zone.as_ref())
        {
            if !record.ends_with(dns_zone) {
                bail!(format!(
                    "Record ({}) does not belong in Zone({}).",
                    record, dns_zone
                ));
            }

            let record = if record.starts_with("_dnslink.") {
                record.to_string()
            } else {
                format!("_dnslink.{}", record)
            };

            let client = Client::new(
                Credentials::UserAuthToken {
                    token: api_token.to_string(),
                },
                HttpApiClientConfig::default(),
                Environment::Production,
            )?;

            // Check if record already exists. Create if it doesn't, update if it does

            let zone_resp = client
                .request(&ListZones {
                    params: ListZonesParams {
                        name: Some(dns_zone.to_string()),
                        ..Default::default()
                    },
                })
                .await?;

            let list_resp = client
                .request(&ListDnsRecords {
                    zone_identifier: &zone_resp.result[0].id,
                    params: ListDnsRecordsParams {
                        name: Some(record.to_string()),
                        ..Default::default()
                    },
                })
                .await?;

            // Filtering for TXT here as ListDnsRecordParams cannot properly set the record type
            let records = list_resp
                .result
                .iter()
                .filter(|r| {
                    std::mem::discriminant(&r.content)
                        == std::mem::discriminant(&DnsContent::TXT {
                            content: "".to_string(),
                        })
                })
                .collect::<Vec<&DnsRecord>>();

            let content = DnsContent::TXT {
                content: format!("dnslink=/ipfs/{}", hash),
            };

            let _resp = match &records[..] {
                [] => {
                    client
                        .request(&CreateDnsRecord {
                            zone_identifier: &zone_resp.result[0].id,
                            params: CreateDnsRecordParams {
                                name: record.as_str(),
                                content,
                                ttl: None,
                                priority: None,
                                proxied: None,
                            },
                        })
                        .await
                }
                [current_rec] => {
                    client
                        .request(&UpdateDnsRecord {
                            zone_identifier: &zone_resp.result[0].id,
                            params: UpdateDnsRecordParams {
                                name: record.as_str(),
                                content,
                                ttl: None,
                                proxied: None,
                            },
                            identifier: current_rec.id.as_str(),
                        })
                        .await
                }
                _ => bail!(format!(
                    "Not updating dnslink, multiple existing records found for {}",
                    record
                )),
            }?;
            Ok(())
        } else {
            unreachable!()
        }
    }
}
