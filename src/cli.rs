use clap::Parser;
use url::Url;

const GITLAB_API_PATH: &str = "/api/v4";

#[derive(Debug, Clone)]
pub enum ScanType {
    Full,
    Group(i32),
    Project(i32),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(
        short = 't',
        long,
        required = true,
        help = "Your Gitlab Personal Access Token, can be created in your Gitlab account settings."
    )]
    pub gitlab_token: String,

    #[arg(
        short = 'u',
        long,
        required = true,
        help = "The URL of the Gitlab instance to be scanned, e.g. https://gitlab.com."
    )]
    pub instance_url: String,

    #[arg(
        short = 'f',
        long,
        required = false,
        help = "The Gitlab instance will be scanned for both groups and projects."
    )]
    pub full_scan: bool,

    #[arg(
        short = 'g',
        long,
        required = false,
        help = "The id of the group to be scanned, e.g. 123."
    )]
    pub group_scan: Option<i32>,

    #[arg(
        short = 'p',
        long,
        required = false,
        help = "The id of the project to be scanned, e.g. 10997."
    )]
    pub project_scan: Option<i32>,
}

pub trait ArgsValidation {
    fn gitlab_token(&self) -> Result<&String, String>;
    fn instance_url(&self) -> Result<Url, String>;
    fn scan_type(&self) -> Result<ScanType, String>;
}

impl ArgsValidation for Args {
    fn gitlab_token(&self) -> Result<&String, String> {
        if self.gitlab_token.len() != 26 {
            Err("Token must be 26 characters".to_string())
        } else if !self.gitlab_token.starts_with("glpat-") {
            Err("Token must start with 'glpat-'".to_string())
        } else {
            Ok(&self.gitlab_token)
        }
    }

    fn instance_url(&self) -> Result<Url, String> {
        self.instance_url
            .parse::<Url>()
            .map_err(|e| format!("Invalid URL: {}", e))
            .and_then(|url| {
                if url.scheme() == "http" || url.scheme() == "https" {
                    let url = if url.path().ends_with('/') {
                        let mut url = url.clone();
                        let path = url.path().to_string();
                        url.set_path(&path[..path.len()-1]);
                        url
                    } else {
                        url
                    };
                    let url = url.join(GITLAB_API_PATH).map_err(|_| "The URL value does not follow an URL scheme and cannot be concatenated with the Gitlab API Path")?;
                    Ok(url)
                } else {
                    Err("The URL value does not follow an URL scheme".to_string())
                }
            })
    }

    fn scan_type(&self) -> Result<ScanType, String> {
        let scan_types = [
            (self.full_scan, "full"),
            (self.group_scan.is_some(), "group"),
            (self.project_scan.is_some(), "project"),
        ];

        let enabled_scans: Vec<_> = scan_types.iter().filter(|(enabled, _)| *enabled).collect();

        match enabled_scans.len() {
            0 => Err("You must specify exactly one scan type: --full-scan, --group-scan, or --project-scan".to_string()),
            1 => {
                match enabled_scans[0].1 {
                    "full" => Ok(ScanType::Full),
                    "group" => {
                        let id = self.group_scan.unwrap();
                        if id > 0 {
                            Ok(ScanType::Group(id))
                        } else {
                            Err("Group ID must be a positive integer".to_string())
                        }
                    },
                    "project" => {
                        let id = self.project_scan.unwrap();
                        if id > 0 {
                            Ok(ScanType::Project(id))
                        } else {
                            Err("Project ID must be a positive integer".to_string())
                        }
                    },
                    _ => unreachable!(),
                }
            },
            _ => Err("Only one scan type can be specified: --full-scan, --group-scan, or --project-scan".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct ValidatedArgs {
    pub gitlab_token: String,
    pub instance_url: Url,
    pub scan_type: ScanType,
}

pub fn validate_args(args: &Args) -> Result<ValidatedArgs, String> {
    let gitlab_token = args.gitlab_token()?.clone();
    let instance_url = args.instance_url()?;
    let scan_type = args.scan_type()?;

    Ok(ValidatedArgs {
        gitlab_token,
        instance_url,
        scan_type,
    })
}

pub fn return_args(args: Args) -> ValidatedArgs {
    validate_args(&args).expect("Invalid arguments")
}
