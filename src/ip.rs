use std::net::IpAddr;

#[derive(Default)]
pub struct BlockingClient {
    client: reqwest::blocking::Client,
}

trait GetBlocking {
    fn get(&self, url: &str) -> Result<String, reqwest::Error>;
}

impl GetBlocking for BlockingClient {
    fn get(&self, url: &str) -> Result<String, reqwest::Error> {
        self.client.get(url).send()?.text()
    }
}

// not async, blocking request
pub(crate) fn get_public_ip<Client: GetBlocking>(
    client: Client,
    url: &str,
) -> Result<IpAddr, reqwest::Error> {
    let ip = client.get(url)?;
    Ok(ip.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use super::*;

    #[derive(Debug)]
    struct MockClient {
        ip: String,
    }

    impl GetBlocking for MockClient {
        fn get(&self, _url: &str) -> Result<String, reqwest::Error> {
            Ok(self.ip.clone())
        }
    }

    #[test]
    fn test_ipv4() {
        let client = MockClient {
            ip: "1.1.1.1".to_string(),
        };

        assert_eq!(
            get_public_ip(client, "any").unwrap(),
            Ipv4Addr::new(1, 1, 1, 1)
        )
    }

    #[test]
    fn test_ipv6() {
        let client = MockClient {
            ip: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(),
        };

        assert_eq!(
            get_public_ip(client, "any").unwrap(),
            Ipv6Addr::new(0x2001, 0xdb8, 0x85a3, 0x0, 0x0, 0x8a2e, 0x0370, 0x7334)
        )
    }
}
