use clap::Parser;
use url::Url;

const GITLAB_API_PATH: &str = "/api/v4";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(
        short = 't',
        long,
        help = "Your Gitlab Personal Access Token, you can get it from your Gitlab account settings."
    )]
    pub gitlab_token: String,

    #[arg(
        short = 'u',
        long,
        help = "Your Gitlab instance URL, e.g. https://example.com."
    )]
    pub instance_url: String,
}

pub trait ArgsValidation {
    fn gitlab_token(&self) -> Result<&String, String>;
    fn instance_url(&self) -> Result<Url, String>;
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
                    let url = url.join(GITLAB_API_PATH).map_err(|_| "The URL value does not follow an URL scheme and cannot be concatenated with the Gitlab API Path")?;
                    Ok(url)
                } else {
                    Err("The URL value does not follow an URL scheme".to_string())
                }
            })
    }
}
