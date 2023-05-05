extern crate clap;

use parser::parse_ips;
use ipnet::IpNet;
use clap::Parser;
use std::net::IpAddr;
use url::Url;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(short, long, value_parser = clap::value_parser!(IpAddr))]
    ip: IpAddr,
    #[arg(short,long, value_parser = clap::value_parser!(Url))]
    ranges_url: Url,
    #[arg(short,long)]
    filter_key: Option<String>
}

fn main() {

    let cli = Cli::parse();

    let ip_ranges: Vec<IpNet> = parse_ips(cli.ranges_url).unwrap();

    println!("Processing concluded with {} IP networks parsed!", ip_ranges.len());
}
