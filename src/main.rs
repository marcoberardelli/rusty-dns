use clap::Parser;
mod dns;

const GET_IP_URL: &str = "http://api.ipify.org?format=json";
// Set the interval in minutes
const INTERVAL_MINUTES: u64 = 1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// Your domain name
    #[arg(short, long)]
    domain: String,

    /// API endpoint to update DNS
    #[arg(short, long)]
    api_url: String,

    /// Bearer token
    #[arg(short, long)]
    token: String,

    /// API endpoint to retreive public IP address
    #[arg(short, long, default_value_t = String::from(GET_IP_URL))]
    ip_api: String,

    /// How often the IP address should be checked in minutes
    #[arg(short, long, default_value_t = INTERVAL_MINUTES)]
    period: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let dns = dns::DnsUpdater {
        domain: args.domain,
        url: args.url,
        token: args.token,
        ip_api: args.ip_api,
        period: args.period,
    };

    dns::dns_updater_thread(dns).await;
}
