use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;
    Ok(articles)
}

fn main() {
    let url = "https://newsapi.org/v2/everything?q=tesla&from=2024-04-28&sortBy=publishedAt&apiKey=5d4fb7c9ae6147f8bb60a2843177a391";
    match get_articles(url) {
        Ok(articles) => {
            for article in articles.articles {
                println!("Title: {}, URL: {}", article.title, article.url);
            }
        }
        Err(e) => eprintln!("Failed to get articles: {}", e),
    }
}
