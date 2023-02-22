use crate::result::CloudflareError;

use cloudflare::{
    endpoints::dns::{self, DnsContent, DnsRecord},
    framework::{apiclient::ApiClient, auth::Credentials, HttpApiClient, HttpApiClientConfig},
};

fn get_record<ApiClientType: ApiClient>(
    api_client: &ApiClientType,
    list_dns_record: &dns::ListDnsRecords,
    new_content: &dns::DnsContent,
) -> Result<DnsRecord, CloudflareError> {
    let response = api_client
        .request(list_dns_record)
        .map_err(CloudflareError::ApiError)?;

    // move to drain_filter once stable
    let (mut drained, _): (Vec<_>, Vec<_>) = response.result.into_iter().partition(|record| {
        std::mem::discriminant(&record.content) == std::mem::discriminant(new_content)
    });

    if drained.len() != 1 {
        return Err(CloudflareError::MoreThanOneRecordFound);
    }

    Ok(drained.remove(0))
}

fn update_a_record<ApiClientType: ApiClient>(
    api_client: &ApiClientType,
    update_record: &dns::UpdateDnsRecord,
) -> Result<String, CloudflareError> {
    Ok(api_client
        .request(update_record)
        .map_err(CloudflareError::ApiError)?
        .result
        .id)
}

fn compare_dns_content(a: &DnsContent, b: &DnsContent) -> bool {
    // DnsContent does not implement Eq trait
    match (a, b) {
        (DnsContent::A { content: a }, DnsContent::A { content: b }) => a == b,
        (DnsContent::AAAA { content: a }, DnsContent::AAAA { content: b }) => a == b,
        (DnsContent::CNAME { content: a }, DnsContent::CNAME { content: b }) => a == b,
        (_, _) => true,
    }
}

pub(crate) fn new_client(token: &str) -> Result<HttpApiClient, CloudflareError> {
    let credentials: Credentials = Credentials::UserAuthToken {
        token: token.to_string(),
    };

    HttpApiClient::new(
        credentials,
        HttpApiClientConfig::default(),
        cloudflare::framework::Environment::Production,
    )
    .map_err(|err| CloudflareError::InvalidHttpClient(err.to_string()))
}

pub(crate) fn update_record<ApiClientType: ApiClient>(
    api_client: &ApiClientType,
    zone_identifier: &str,
    dns_record_name: &str,
    new_content: dns::DnsContent,
) -> Result<String, CloudflareError> {
    let record = get_record(
        api_client,
        &dns::ListDnsRecords {
            zone_identifier,
            params: dns::ListDnsRecordsParams {
                record_type: None,
                name: Some(dns_record_name.to_string()),
                page: None,
                per_page: None,
                order: None,
                direction: None,
                search_match: None,
            },
        },
        &new_content,
    )?;

    if !compare_dns_content(&record.content, &new_content) {
        update_a_record(
            api_client,
            &dns::UpdateDnsRecord {
                zone_identifier,
                identifier: record.id.as_ref(),
                params: dns::UpdateDnsRecordParams {
                    ttl: Some(record.ttl),
                    proxied: Some(true),
                    name: dns_record_name,
                    content: new_content,
                },
            },
        )
    } else {
        println!("[INFO] DNS record up-to-date");
        Ok(record.id)
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use cloudflare::{
        endpoints::dns,
        framework::{apiclient::ApiClient, endpoint::Method, response::ApiSuccess},
    };

    use super::*;

    struct MockApiClient {
        dns_records_output: String,
        update_records_output: String,
    }

    impl ApiClient for MockApiClient {
        fn request<ResultType, QueryType, BodyType>(
            &self,
            endpoint: &dyn cloudflare::framework::endpoint::Endpoint<
                ResultType,
                QueryType,
                BodyType,
            >,
        ) -> cloudflare::framework::response::ApiResponse<ResultType>
        where
            ResultType: cloudflare::framework::response::ApiResult,
            QueryType: serde::Serialize,
            BodyType: serde::Serialize,
        {
            let parsed_json: ApiSuccess<ResultType> = match endpoint.method() {
                Method::Get => serde_json::from_str(&self.dns_records_output).unwrap(),
                Method::Put => serde_json::from_str(&self.update_records_output).unwrap(),
                _ => unimplemented!(),
            };
            Ok(parsed_json)
        }
    }

    #[test]
    fn get_one_record() {
        let mock_client = MockApiClient {
            dns_records_output: r#"{
  "success": true,
  "errors": [],
  "messages": [],
  "result": [
    {
      "id": "372e67954025e0ba6aaa6d586b9e0b59",
      "type": "AAAA",
      "name": "example.com",
      "content": "2001:0db8:85a3:0000:0000:8a2e:0370:7334",
      "proxiable": true,
      "proxied": false,
      "comment": "Domain verification record",
      "tags": [
        "owner:dns-team"
      ],
      "ttl": 3600,
      "locked": false,
      "zone_id": "023e105f4ecef8ad9ca31a8372d0c353",
      "zone_name": "example.com",
      "created_on": "2014-01-01T05:20:00.12345Z",
      "modified_on": "2014-01-01T05:20:00.12345Z",
      "data": {},
      "meta": {
        "auto_added": true,
        "source": "primary"
      }
    },
    {
      "id": "372e67954025e0ba6aaa6d586b9e0b59",
      "type": "A",
      "name": "example.com",
      "content": "198.51.100.4",
      "proxiable": true,
      "proxied": false,
      "comment": "Domain verification record",
      "tags": [
        "owner:dns-team"
      ],
      "ttl": 3600,
      "locked": false,
      "zone_id": "023e105f4ecef8ad9ca31a8372d0c353",
      "zone_name": "example.com",
      "created_on": "2014-01-01T05:20:00.12345Z",
      "modified_on": "2014-01-01T05:20:00.12345Z",
      "data": {},
      "meta": {
        "auto_added": true,
        "source": "primary"
      }
    }
  ]
}"#
            .to_string(),
            update_records_output: r#"{
  "success": true,
  "errors": [],
  "messages": [],
  "result": {
      "id": "372e67954025e0ba6aaa6d586b9e0b59",
      "type": "A",
      "name": "example.com",
      "content": "1.1.1.1",
      "proxiable": true,
      "proxied": false,
      "comment": "Domain verification record",
      "tags": [
        "owner:dns-team"
      ],
      "ttl": 3600,
      "locked": false,
      "zone_id": "023e105f4ecef8ad9ca31a8372d0c353",
      "zone_name": "example.com",
      "created_on": "2014-01-01T05:20:00.12345Z",
      "modified_on": "2014-01-01T05:20:00.12345Z",
      "data": {},
      "meta": {
        "auto_added": true,
        "source": "primary"
      }
    }
}"#
            .to_string(),
        };

        let new_content = dns::DnsContent::A {
            content: Ipv4Addr::new(1, 1, 1, 1),
        };

        let record = update_record(
            &mock_client,
            "023e105f4ecef8ad9ca31a8372d0c353",
            "example.com",
            new_content,
        )
        .unwrap();

        assert_eq!(record, "372e67954025e0ba6aaa6d586b9e0b59");
    }
}
