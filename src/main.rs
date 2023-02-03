extern crate clap;

use parser::parse_ips;
use ipnet::IpNet;
use clap::Parser;
use std::net::IpAddr;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(short, long, value_parser = clap::value_parser!(IpAddr))]
    ip: String,
    #[arg(short,long)]
    ranges_url: String,
    #[arg(short,long)]
    filter_key: Option<String>
}

fn main() {

    let cli = Cli::parse();

    let googlebot_ip_range_url: &'static str = "https://developers.google.com/static/search/apis/ipranges/googlebot.json";

    let google_ip_ranges: Vec<IpNet> = parse_ips(googlebot_ip_range_url).unwrap();

    println!("Processing concluded with {} IP networks parsed!", google_ip_ranges.len());
}
