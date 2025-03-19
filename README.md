# GitlabAuditor

Quickly audit your Gitlab instance pipelines logs to find secrets.
This project aim to fetch all logs in your gitlab pipelines.

## How to use ?

```bash
# Build the tool
cargo build --release --target-dir .

# Run the tool
cd /release
./gitlab_auditor -u <http://my-random-gitlab-domain.com> -t <my-personnal-token-from-gitlab> -f

# Get help
./gitlab_auditor --help
```

## RoadMap

- [x] Full scan
- [ ] Group scan
- [ ] Project scan
- [ ] Secret detection
