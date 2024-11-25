use std::fs::read_to_string;
use reqwest::blocking::Client;
use scraper::{Html, Selector};

#[cfg_attr(test, mockall::automock)]
pub trait AdventOfCodeClient {
    fn get_problem_statement(&self, year: u16, day: u8) -> Result<String, Box<dyn std::error::Error>>;
    fn extract_problem_name(&self, html: &str) -> Result<String, Box<dyn std::error::Error>>;
    fn get_problem_input(&self, year: u16, day: u8) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct HttpAdventOfCodeClient {
    client: Client,
}

impl HttpAdventOfCodeClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = if let Some(session) = get_session_cookie() {
            let cookie = format!("session={}", session);
            Client::builder()
                .default_headers(
                    std::iter::once((
                        reqwest::header::COOKIE,
                        reqwest::header::HeaderValue::from_str(&cookie)?
                    )).collect()
                ).build()?
        } else {
            Client::builder().build()?
        };

        Ok(Self { client })
    }
}

impl AdventOfCodeClient for HttpAdventOfCodeClient {
    fn get_problem_statement(&self, year: u16, day: u8) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("https://adventofcode.com/{}/day/{}", year, day);
        let response = self.client.get(&url).send()?;
        let html = response.text()?;
        
        let document = Html::parse_document(&html);
        let selector = Selector::parse("article.day-desc").unwrap();
        
        let mut content = String::new();
        for article in document.select(&selector) {
            content.push_str(&article.html());
            content.push_str("\n\n");
        }
        
        Ok(content)
    }

    fn extract_problem_name(&self, html: &str) -> Result<String, Box<dyn std::error::Error>> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("article.day-desc > h2").unwrap();
        
        document.select(&selector)
            .next()
            .and_then(|h2| {
                h2.text()
                    .collect::<String>()
                    .split(':')
                    .nth(1)
                    .map(|s| s.trim_end_matches('-').trim().to_string())
            })
            .ok_or_else(|| "Could not find problem title".into())
    }

    fn get_problem_input(&self, year: u16, day: u8) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let response = self.client.get(&url).send()?;
        Ok(response.text()?)
    }
}

fn get_session_cookie() -> Option<String> {
    read_to_string(".session").ok()
        .map(|s| s.trim().to_string())
}

#[cfg(test)]
pub struct FakeClient {
    problem_statement: String,
    problem_name: String,
}

#[cfg(test)]
impl FakeClient {
    pub fn new(problem_statement: &str, problem_name: &str) -> Self {
        Self {
            problem_statement: problem_statement.to_string(),
            problem_name: problem_name.to_string(),
        }
    }
}

#[cfg(test)]
impl AdventOfCodeClient for FakeClient {
    fn get_problem_statement(&self, _year: u16, _day: u8) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.problem_statement.clone())
    }

    fn extract_problem_name(&self, _html: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.problem_name.clone())
    }

    fn get_problem_input(&self, _year: u16, _day: u8) -> Result<String, Box<dyn std::error::Error>> {
        Ok("fake input".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_problem_name() {
        let client = HttpAdventOfCodeClient::new().unwrap();
        let html = r#"<!DOCTYPE html><html><body><article class="day-desc"><h2>--- Day 1: Test Problem ---</h2></article></body></html>"#;
        assert_eq!(client.extract_problem_name(html).unwrap(), "Test Problem");
    }

    #[test]
    fn test_fake_client() {
        let client = FakeClient::new("test html", "Test Problem");
        assert_eq!(client.get_problem_statement(2024, 1).unwrap(), "test html");
        assert_eq!(client.extract_problem_name("ignored").unwrap(), "Test Problem");
    }
} 