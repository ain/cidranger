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

fn main() -> Result<(), String> {

    let cli = Cli::parse();

    let ip_ranges: Vec<IpNet> = parse_ips(cli.ranges_url).unwrap();

    let mut ip_match = false;

    for ip_range in ip_ranges {
        if ip_range.contains(&cli.ip) {
            println!("{} is a match for {}!", cli.ip, ip_range);
            ip_match = true;
        }
    }

    if ip_match {
        println!("{} was matched!", cli.ip);
        Ok(())
    } else {
        let error_message = format!("{} not found in any of the ranges!", cli.ip);
        return Err(error_message)
    }
}
