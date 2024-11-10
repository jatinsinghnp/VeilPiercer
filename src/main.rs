use reqwest::Error;
use serde::Deserialize;
use std::env;
use std::process::exit;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

#[derive(Deserialize)]
struct IpInfo {
    ip: String,
    country: Option<String>,
    region: Option<String>,
    city: Option<String>,
    org: Option<String>,
}

#[derive(Deserialize)]
struct IpApiInfo {
    status: String,
    country: Option<String>,
    regionName: Option<String>,
    city: Option<String>,
    isp: Option<String>,
    query: Option<String>,
}

async fn fetch_ip_info(ip: &str) -> Result<IpInfo, Error> {
    let url = format!("http://ipinfo.io/{}/json", ip);
    let token = "9bcc497143820e"; // Replace with your own token if needed
    let url_with_token = format!("{}?token={}", url, token);
    let response = reqwest::get(&url_with_token).await?;
    let ip_info = response.json::<IpInfo>().await?;
    Ok(ip_info)
}

async fn fetch_ip_api_info(ip: &str) -> Result<IpApiInfo, Error> {
    let url = format!("http://ip-api.com/json/{}", ip);
    let response = reqwest::get(&url).await?;
    let ip_api_info = response.json::<IpApiInfo>().await?;
    Ok(ip_api_info)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <domain>", args[0]);
        exit(1);
    }

    let domain = &args[1];
    
    // Use trust-dns-resolver to resolve the IP
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())
        .expect("Failed to create resolver");
    let response = resolver.lookup_ip(domain).await;

    let ip = match response {
        Ok(ips) => match ips.iter().next() {
            Some(ip) => ip.to_string(),
            None => {
                eprintln!("No IP addresses found for domain: {}", domain);
                exit(1);
            }
        },
        Err(_) => {
            eprintln!("Failed to resolve domain: {}", domain);
            exit(1);
        }
    };

    println!("Target Domain: {}", domain);
    println!("Resolved Cloudflare IP: {}", ip);

    match fetch_ip_info(&ip).await {
        Ok(info) => {
            println!("--- IPInfo ---");
            println!("IP: {}", info.ip);
            if let Some(country) = info.country {
                println!("Country: {}", country);
            }
            if let Some(region) = info.region {
                println!("Region: {}", region);
            }
            if let Some(city) = info.city {
                println!("City: {}", city);
            }
            if let Some(org) = info.org {
                println!("Organization: {}", org);
            }
        }
        Err(e) => eprintln!("Failed to fetch IPInfo data: {}", e),
    }

    match fetch_ip_api_info(&ip).await {
        Ok(info) => {
            println!("--- IP-API ---");
            if info.status == "success" {
                if let Some(country) = info.country {
                    println!("Country: {}", country);
                }
                if let Some(region) = info.regionName {
                    println!("Region: {}", region);
                }
                if let Some(city) = info.city {
                    println!("City: {}", city);
                }
                if let Some(isp) = info.isp {
                    println!("ISP: {}", isp);
                }
                if let Some(ip) = info.query {
                    println!("Query IP: {}", ip);
                }
            } else {
                println!("IP-API failed to retrieve data for this IP");
            }
        }
        Err(e) => eprintln!("Failed to fetch IP-API data: {}", e),
    }
}


