use argh::FromArgs;
use cloudflareddns::Updater;

#[derive(FromArgs)]
/// Cloudflare ddns configuration
struct Config {
    /// zone ID of the domain
    #[argh(option, short = 'z')]
    zone_id: String,

    /// cloudflare token
    #[argh(option, short = 't')]
    token: String,

    /// record name to update
    #[argh(option)]
    record_name: String,

    /// url to fetch the public ip from
    #[argh(option)]
    ip_checker: Option<String>,
}

fn main() {
    let conf: Config = argh::from_env();

    let updater = match conf.ip_checker {
        Some(custom_url) => Updater::new(custom_url),
        None => Updater::default(),
    };

    updater
        .update(&conf.token, &conf.zone_id, &conf.record_name)
        .unwrap();
}
