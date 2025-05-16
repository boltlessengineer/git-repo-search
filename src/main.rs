use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GitHubSearchResponse {
    items: Vec<GitHubRepo>,
}

#[derive(Debug, Deserialize)]
struct GitHubRepo {
    full_name: String,
    stargazers_count: u32,
    language: Option<String>,
    clone_url: String,
    // description: Option<String>,
    // forks_count: u32,
    // open_issues_count: u32,
}

async fn search_github_repos(query: &str, count: u32) -> Result<Vec<GitHubRepo>, Error> {
    let url = format!("https://api.github.com/search/repositories?q={query}&per_page={count}");

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header("User-Agent", "rust-lang-app")
        .send()
        .await?
        .json::<GitHubSearchResponse>()
        .await?;

    Ok(res.items)
}

#[tokio::main]
async fn main() {
    let query = std::env::args().nth(1).unwrap_or_else(|| {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    });
    match search_github_repos(&query, 5).await {
        Ok(mut repos) => {
            repos.sort_by_key(|r| u32::MAX - r.stargazers_count);
            for GitHubRepo {
                full_name,
                clone_url,
                stargazers_count,
                language
            } in repos
            {
                // let description = description.unwrap_or(String::from("<no description>"));
                let language = language.as_deref().unwrap_or("-");
                println!("{full_name:<32} (⭐{stargazers_count:>6}) {language:<10} {clone_url}");
            }
        }
        Err(err) => eprintln!("Error occured: {err}"),
    }
}
