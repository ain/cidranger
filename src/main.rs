use parser::parse_ips;
use ipnet::IpNet;

fn main() {

    let googlebot_ip_range_url: &'static str = "https://developers.google.com/static/search/apis/ipranges/googlebot.json";

    //let arg = std::env::args()
        //.skip(1)
        //.next()
        //.expect("one argument required");

    let google_ip_ranges: Vec<IpNet> = parse_ips(googlebot_ip_range_url).unwrap();

    println!("Processing concluded with {} IP networks parsed!", google_ip_ranges.len());
}
