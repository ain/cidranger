use serde::Deserialize;
use reqwest::Client;
use tokio::runtime::Runtime;
use std::collections::HashMap;
use ipnet::IpNet;
use url::Url;

#[derive(Deserialize, Debug)]
struct IpCollection {
    creationTime: String,
    prefixes: Vec<HashMap<String, String>>
}

pub fn parse_ips(url: Url) -> Option<Vec<IpNet>> {

    println!("Fetching {}...", url);

    let client = Client::new();
    let rt = Runtime::new().unwrap();
    let response = rt.block_on(async {
        let res = client.get(url).send().await?;
        res.json::<IpCollection>().await
    });

    match response {
        Ok(json) => {
            let mut ranges : Vec<IpNet> = Vec::new();
            for prefix in json.prefixes {
                let range : IpNet = match (prefix.get("ipv4Prefix"), prefix.get("ipv6Prefix")) {
                    (Some(ipv4), _) => ipv4.to_string().parse().unwrap(),
                    (_, Some(ipv6)) => ipv6.to_string().parse().unwrap(),
                    _ => continue,
                };
                ranges.push(range);
            }
            Some(ranges)
        },
        Err(e) => {
            println!("An error occurred: {:?}", e);
            None
        }
    }
}
