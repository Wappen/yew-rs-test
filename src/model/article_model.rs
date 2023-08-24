use chrono::{DateTime, Utc};
use std::fmt::Debug;

pub type ArticleId = i32;

#[derive(Clone, Debug, PartialEq)]
pub struct ArticleModel {
    pub id: ArticleId,
    pub title: String,
    pub author: String,
    pub text: String,
    pub thumbnail: String,
    pub header_img: String,
    pub published: Option<DateTime<Utc>>,
}

impl ArticleModel {
    pub fn subtitle(&self) -> String {
        match self.published {
            Some(published) => format!(
                "by {} on {}",
                self.author,
                published.format("%a, %d. %b %Y")
            ),
            None => format!("by {}", self.author),
        }
    }
}
