#[cfg(test)]

mod tests {

    use ipnet::IpNet;

    #[test]
    fn check_network_count() {
        let url = "https://raw.githubusercontent.com/ain/cidranger/main/tests/data/sample.json";
        let ipnet: Option<Vec<IpNet>> = parser::parse_ips(url);
        assert_eq!(6, ipnet.expect("correct network count").len());
    }
}
