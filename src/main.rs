use clap::Parser;
mod dns;

const GET_IP_URL: &str = "http://api.ipify.org?format=json";
// Set the interval in minutes
const INTERVAL_MINUTES: u64 = 1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    zone_id: String,

    /// Number of times to greet
    #[arg(short, long)]
    record_id: String,

    /// Bearer token
    #[arg(short, long)]
    token: String,

    #[arg(short, long, default_value_t = String::from(GET_IP_URL))]
    ip_api: String,

    #[arg(short, long, default_value_t = INTERVAL_MINUTES)]
    period: u64
}



#[tokio::main]
async fn main() {

    let args = Args::parse();

    let dns = dns::DnsUpdater {
        zone_id: args.zone_id,
        record_id: args.record_id,
        token: args.token,
        ip_api: args.ip_api,
        period: args.period
    };


    dns::dns_updater_thread(dns).await;
}
