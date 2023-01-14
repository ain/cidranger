//use serde_json::Value;
use serde::Deserialize;
//use serde_json::Result;
use reqwest::Client;
use tokio::runtime::Runtime;
//use exitfailure::ExitFailure;
//use std::env;
//use std::net::Ipv4Addr;
//use std::net::Ipv6Addr;
//use std::fmt;
use std::collections::HashMap;
use std::net::SocketAddr;


//impl parse_ips()<GooglebotIpRange, reqwest::Error> {
//#[derive(Deserialize, Debug)]
//struct Prefix {
   //ipv6Prefix: String
//}

#[derive(Deserialize, Debug)]
pub struct IpCollection {
    pub creationTime: String,
    //prefixes: Vec<Prefix>
    prefixes: Vec<HashMap<String, String>>
}

pub fn parse_ips(url: &str) -> Option<IpCollection> {

    println!("fetching {}", url);

    let client = Client::new();
    let rt = Runtime::new().unwrap();
    let response = rt.block_on(async {
        let res = client.get(url).send().await?;
        res.json::<IpCollection>().await
    });

    match response {
        Ok(json) => Some(json),
        Err(e) => {
            println!("An error occurred: {:?}", e);
            None
        }
    }

    //#[derive(Deserialize)]
    //struct GooglebotIpRange {
        //kind: String,
        //prefixes: Vec<IpCollection>
    //}


    //return serde_json::from_str(&json_str).unwrap();
}
