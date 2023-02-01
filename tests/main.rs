#[cfg(test)]

mod tests {

    use ipnet::IpNet;

    #[test]
    fn check_parse_ips() {
        let url = "https://raw.githubusercontent.com/ain/cidranger/main/tests/data/sample.json";
        let ipnet: Option<Vec<IpNet>> = parser::parse_ips(url);
        assert_eq!("2001:4860:4801:10::/64, 2001:4860:4801:c::/64, 2001:4860:4801:f::/64, 34.100.182.96/28, 34.146.150.144/28, 66.249.79.96/27", ipnet.unwrap().iter().map(|range| range.to_string()).collect::<Vec<String>>().join(", "));
    }
}
