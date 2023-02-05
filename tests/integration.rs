extern crate assert_cmd;

//static INVALID_URL_OUTPUT: &'static str = "Got error: failed to lookup address information:";

mod integration {

    use assert_cmd::Command;
    use predicates::prelude::*;
    //use INVALID_URL_OUTPUT;

    #[test]
    fn cli_missing_all_required_options() {
        let mut cmd = Command::cargo_bin("cidranger").unwrap();
        cmd.assert().failure();
    }

    #[test]
    fn cli_missing_ip() {
        let mut cmd = Command::cargo_bin("cidranger").unwrap();
        let assert = cmd.arg("--ranges-url=https://raw.githubusercontent.com/ain/cidranger/main/tests/data/sample.json").assert();
        assert.failure().stderr(predicate::str::contains("error: the following required arguments were not provided:\n  --ip <IP>\n\nUsage: cidranger --ip <IP> --ranges-url <RANGES_URL>\n\nFor more information, try '--help'."));
    }

    #[test]
    fn cli_missing_ranges() {
        let mut cmd = Command::cargo_bin("cidranger").unwrap();
        let assert = cmd.arg("--ip=127.0.0.1").assert();
        assert.failure();
    }

}
