use cloudflare::endpoints::dns;
use ip::get_public_ip;
use record::update_record;
use std::error::Error;

mod ip;
mod record;
mod result;

pub fn update(
    fetch_public_ip_url: &str,
    token: &str,
    zone_identifier: &str,
    dns_record_name: &str,
) -> Result<(), Box<dyn Error>> {
    let ip_client = ip::BlockingClient::default();

    let current_ip = get_public_ip(ip_client, fetch_public_ip_url)?;

    let record_content = match current_ip {
        std::net::IpAddr::V6(ip) => dns::DnsContent::AAAA { content: ip },
        std::net::IpAddr::V4(ip) => dns::DnsContent::A { content: ip },
    };

    let result_id = update_record(token, zone_identifier, dns_record_name, record_content)?;

    println!("[SUCCESS]: Record updated - {}", result_id);

    Ok(())
}
