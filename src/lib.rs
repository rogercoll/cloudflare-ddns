use cloudflare::endpoints::dns;
use ip::get_public_ip;
use record::update_record;
use std::error::Error;

mod ip;
mod record;
mod result;

pub struct Updater {
    fetch_public_ip_url: String,
}

impl Default for Updater {
    fn default() -> Self {
        Updater {
            fetch_public_ip_url: "http://ifconfig.co/json".to_string(),
        }
    }
}

impl Updater {
    pub fn new(fetch_url: String) -> Self {
        Updater {
            fetch_public_ip_url: fetch_url,
        }
    }
    pub fn update(
        &self,
        token: &str,
        zone_identifier: &str,
        dns_record_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        let ip_client = ip::BlockingClient::default();

        let current_ip = get_public_ip(ip_client, &self.fetch_public_ip_url)?;

        println!("[INFO]: Udating record name to - {}", current_ip);

        let record_content = match current_ip {
            std::net::IpAddr::V6(ip) => dns::DnsContent::AAAA { content: ip },
            std::net::IpAddr::V4(ip) => dns::DnsContent::A { content: ip },
        };

        let result_id = update_record(token, zone_identifier, dns_record_name, record_content)?;

        println!("[SUCCESS]: Record updated - {}", result_id);

        Ok(())
    }
}
