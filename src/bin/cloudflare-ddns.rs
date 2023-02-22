use std::time::Duration;

use argh::FromArgs;
use cloudflareddns::Updater;

#[derive(FromArgs)]
/// Cloudflare ddns tool configuration
struct Config {
    /// zone ID of the domain
    #[argh(option, short = 'z')]
    zone_id: String,

    /// cloudflare API token
    #[argh(option, short = 't')]
    token: String,

    /// record name to update
    #[argh(option)]
    record_name: String,

    /// url to fetch the public ip from
    #[argh(option)]
    ip_checker: Option<String>,

    /// seconds interval for long running execution
    #[argh(option, short = 'l')]
    long_running: Option<u64>,
}

fn main() {
    let conf: Config = argh::from_env();

    let updater = match conf.ip_checker {
        Some(custom_url) => Updater::new(custom_url),
        None => Updater::default(),
    };

    if let Some(interval) = conf.long_running {
        loop {
            updater
                .update(&conf.token, &conf.zone_id, &conf.record_name)
                .unwrap();
            std::thread::sleep(Duration::from_secs(interval))
        }
    } else {
        updater
            .update(&conf.token, &conf.zone_id, &conf.record_name)
            .unwrap();
    }
}
