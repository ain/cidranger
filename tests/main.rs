#[cfg(test)]

mod tests {

    #[test]
    fn check_network_count() {
        let url = "https://raw.githubusercontent.com/ain/cidranger/main/tests/data/sample.json";
        let ipnet = parser::parse_ips(url);
        assert_eq!(200, ipnet.expect("correct network count").len());
    }
}
