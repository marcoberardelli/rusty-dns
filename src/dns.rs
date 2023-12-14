use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::Client;
use serde::ser::StdError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct DnsUpdater {
    pub domain: String,
    pub url: String,
    pub token: String,
    pub ip_api: String,
    pub period: u64,
}

/// HTTP request body for updating a DNS record
#[derive(Debug, Serialize)]
struct CloudflareRequest {
    record_type: String,
    content: String, // Ip address
    name: String,    // Domain name
    proxied: bool,
}

/// HTTP response for getting IP address
#[derive(Debug, Deserialize)]
struct IpResponse {
    ip: String,
}

async fn get_ip(get_ip_api: &str) -> Result<IpResponse, Box<dyn StdError>> {
    let client = Client::new();

    let response = client
        .get(get_ip_api)
        .send()
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?
        .text()
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    serde_json::from_str(&response).map_err(|e| Box::new(e) as Box<dyn StdError>)
}

async fn update_dns(url: &str, new_ip: &str, token: &str) -> Result<(), reqwest::Error> {
    // Custom headers
    let mut bearer_token: String = String::from(token);

    if token.contains("Bearer") {
        bearer_token = format!("Bearer {token}");
    }

    let bearer_token_str = bearer_token.as_str();
    let headers = vec![
        ("Authorization", bearer_token_str),
        ("Content-Type", "application/json"),
    ];
    // Map rappresentation of headers
    let mut header_map = HeaderMap::new();
    for (name, value) in headers {
        header_map.insert(
            reqwest::header::HeaderName::from_bytes(name.as_bytes()).unwrap(),
            HeaderValue::from_str(value).unwrap(),
        );
    }

    // Create http request
    let client = Client::new();
    let req = CloudflareRequest {
        record_type: String::from("A"),
        content: String::from(new_ip),
        name: String::from("berver.eu"),
        proxied: true,
    };
    // Converting CloudflareRequest to json string
    let body = serde_json::to_string(&req)
        .expect("Failed to creating json")
        .replace("record_", "");

    // Send PUT request with custom headers
    client
        .put(url)
        .headers(header_map)
        .body(body)
        .send()
        .await?;

    Ok(())
}

pub async fn dns_updater_thread(param: DnsUpdater) {
    let mut old_ip = String::new();

    loop {
        // Get current IP
        match get_ip(&param.ip_api).await {
            Ok(ip) => {
                // Compare with old IP
                if ip.ip != old_ip {
                    println!("Updating [{}] from old ip [{}] to [{}]", param.domain, old_ip, ip.ip);
                    // Update DNS with new ip
                    if let Err(err) = update_dns(&param.url, &ip.ip, &param.token).await {
                        eprintln!("Error: {}", err);
                    } else {
                        old_ip = ip.ip;
                        println!("Successfully updated IP address to [{}]", old_ip);
                    }
                } else {
                    println!("Ip did not change ({})", old_ip);
                }
            }
            Err(err) => eprintln!("Failed to get ip address: {}", err),
        }

        // Sleep for the specified interval
        tokio::time::sleep(Duration::from_secs(param.period * 60)).await;
    }
}
