use argh::FromArgs;
use cloudflareddns::Updater;
use crossbeam_channel::{bounded, select, tick, Receiver};
use std::time::Duration;

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

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
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

    // long running execution
    if let Some(interval) = conf.long_running {
        let ctrl_c_events = ctrl_channel().unwrap();
        let ticks = tick(Duration::from_secs(interval));
        loop {
            select! {
                recv(ticks) -> _ => {
                    updater
                        .update(&conf.token, &conf.zone_id, &conf.record_name)
                        .unwrap();
                }
                recv(ctrl_c_events) -> _ => {
                    println!("Stopping the long running execution!");
                    return;
                }
            }
        }
    }
}
