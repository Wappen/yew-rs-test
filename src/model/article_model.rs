use chrono::{DateTime, Utc};
use std::fmt::Debug;

pub type ArticleId = i32;

#[derive(Clone, Debug, PartialEq)]
pub struct ArticleModel {
    pub id: ArticleId,
    pub title: String,
    pub author: String,
    pub description: String,
    pub text: String,
    pub thumbnail: String,
    pub published: Option<DateTime<Utc>>,
}
