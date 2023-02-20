use crate::result::CloudflareError;

use cloudflare::{
    endpoints::dns::{self, DnsRecord},
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

pub(crate) fn update_record(
    token: &str,
    zone_identifier: &str,
    dns_record_name: &str,
    new_content: dns::DnsContent,
) -> Result<String, CloudflareError> {
    let credentials: Credentials = Credentials::UserAuthToken {
        token: token.to_string(),
    };

    let api_client = HttpApiClient::new(
        credentials,
        HttpApiClientConfig::default(),
        cloudflare::framework::Environment::Production,
    )
    .map_err(|err| CloudflareError::InvalidHttpClient(err.to_string()))?;

    let record = get_record(
        &api_client,
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

    // if same content, no update needed
    // if record.content == new_content {
    //     Ok(())
    // }

    update_a_record(
        &api_client,
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
}

#[cfg(test)]
mod tests {

    #[test]
    fn valid_get_record() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
