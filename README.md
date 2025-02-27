# GitlabAuditor

Quickly audit your Gitlab Instance to find secrets.

## Run the tool (development mode)

```bash
cargo run -- -t glpat-1234567890abcdef1234 -u https://gitlab.com -f
cargo run -- -t glpat-1234567890abcdef1234 -u https://gitlab.com -p 123
cargo run -- -t glpat-1234567890abcdef1234 -u https://gitlab.com -g 1234
```
