use serde::Deserialize;
use reqwest::Client;
use tokio::runtime::Runtime;
use std::collections::HashMap;
use ipnet::IpNet;
use serde_json::from_str;

#[derive(Deserialize, Debug)]
pub struct IpCollection {
    pub creationTime: String,
    pub prefixes: Vec<HashMap<String, String>>
}

impl IpCollection {
    fn create<S: Into<String>>(creationTime: S, prefixes: Vec<HashMap<String, String>>) -> Self where Self: Sized {
        IpCollection{ creationTime: creationTime.into(), prefixes: prefixes }
    }

    fn creationTime(&self) -> &String {
        &self.creationTime
    }

    fn prefixes(&self) -> &Vec<HashMap<String, String>> {
        &self.prefixes
    }
}

pub fn parse_ips(url: &str) -> Option<Vec<IpNet>> {

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
