[![Continuos Integration](https://github.com/rogercoll/cloudflare-ddns/actions/workflows/test.yaml//badge.svg?branch=main)](https://github.com/rogercoll/cloudflare-ddns/actions/workflows/test.yaml?query=branch%3Amain)
[![Code Coverage](https://codecov.io/github/rogercoll/cloudflare-ddns/coverage.svg?branch=main&token=)](https://codecov.io/gh/rogercoll/cloudflare-ddns)
[![dependency status](https://deps.rs/repo/github/rogercoll/cloudflare-ddns/status.svg)](https://deps.rs/repo/github/rogercoll/cloudflare-ddns)

# Cloudflare Dynamic DNS

Tool to update Cloudflare DNS records, it can be used as a long running process.

By default, the tool uses `ifconfig.co/json` site to fetch the current public ip. It can
be overrided with `--ip-checker` configuration option, note that the specified url must
return an IPv4 or IPv6 in its payload with the following JSON schema:

```json
{
    "ip": "1.1.1.1",
    ...
}
```

## Usage

### CLI

```bash
$ cargo run -- --help
Usage: cloudflare-ddns -z <zone-id> -t <token> --record-name <record-name> [--ip-checker <ip-checker>] [-l <long-running>]

Cloudflare ddns tool configuration

Options:
  -z, --zone-id     zone ID of the domain
  -t, --token       cloudflare API token
  --record-name     record name to update
  --ip-checker      url to fetch the public ip from
  -l, --long-running
                    seconds interval for long running execution
  --help            display usage information
  ```

### Container

```bash
 $ podman run -e TOKEN=123 -e RECORD_NAME=example.com -e ZONE_ID=456 -e LONG_RUNNING=20 docker.io/coll97/cloudflare-ddns
```

## DNS Record type support

  - [x] A
  - [x] AAAA
