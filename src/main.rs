extern crate iprange;

use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use iprange::IpRange;
//use ipnet::Ipv4Net;
use parser::parse_ips;
use parser::IpCollection;


fn main() {

    let googlebot_ip_range_url: &'static str = "https://developers.google.com/static/search/apis/ipranges/googlebot.json";

    //let arg = std::env::args()
        //.skip(1)
        //.next()
        //.expect("one argument required");

    let google_ip_ranges: IpCollection = parse_ips(googlebot_ip_range_url).unwrap();
    //parse_ips(googlebot_ip_range_url);

    println!("IPs ranges parsed: {:?}", google_ip_ranges.creationTime);

    //match arg.to_str() {
       //Some(arg) =>
    //}
}
