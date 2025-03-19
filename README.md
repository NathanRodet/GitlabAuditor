# GitlabAuditor

Quickly audit your Gitlab instance pipelines logs to find secrets.  
This project aim to fetch all logs in your gitlab pipelines to use open-source tools and find secrets in them.

## How to use ?

### Gitlab Auditor

```bash
# Build the tool
cargo build --release --target-dir .

# Run the tool
cd /release
./gitlab_auditor -u <http://my-random-gitlab-domain.com> -t <my-personnal-token-from-gitlab> -f

# Get help
./gitlab_auditor --help
```

### Secret Detection with Gitleaks

#### [Get Gitleaks from official repository](https://github.com/gitleaks/gitleaks/releases)

```bash
# Run Gitleak
./gitleaks detect --source "<path-to-results>" --report-format json --report-path gitleaks_results.json --no-git
```

## RoadMap

- [x] Full scan
- [x] Secret detection
- [ ] Group scan
- [ ] Project scan
