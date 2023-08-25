use crate::model::ArticleModel;
use chrono::{DateTime, NaiveDateTime, Utc};

pub trait ArticleRepository {
    fn find_all(&self) -> Vec<ArticleModel>;
    fn find_by_id(&self, id: i32) -> Option<ArticleModel>;
}

impl PartialEq for dyn ArticleRepository {
    fn eq(&self, other: &Self) -> bool {
        self.find_all() == other.find_all()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MockArticleRepository {
    articles: Vec<ArticleModel>,
}

impl ArticleRepository for MockArticleRepository {
    fn find_all(&self) -> Vec<ArticleModel> {
        self.articles.clone()
    }

    fn find_by_id(&self, id: i32) -> Option<ArticleModel> {
        self.articles.iter().find(|a| a.id == id).cloned()
    }
}

impl Default for MockArticleRepository {
    fn default() -> Self {
        MockArticleRepository {
            articles: vec![
                ArticleModel {
                    id: 1,
                    title: "Hello World!".to_string(),
                    author: "Lenz".to_string(),
                    text: include_str!("articles/hello_world.md").to_string(),
                    thumbnail: "/static/img/thumbnail.png".to_string(),
                    header_img: "/static/img/thumbnail.png".to_string(),
                    published: Some(Utc::now()),
                },
                ArticleModel {
                    id: 2,
                    title: "Hello Planet!".to_string(),
                    author: "Snens".to_string(),
                    text: include_str!("articles/hello_planet.md").to_string(),
                    thumbnail: "/static/img/thumbnail2.jpeg".to_string(),
                    header_img: "/static/img/thumbnail2.jpeg".to_string(),
                    published: Some(date_from_str("2020-04-20")),
                },
                ArticleModel {
                    id: 3,
                    title: "Hello Universe!".to_string(),
                    author: "Wappen".to_string(),
                    text: include_str!("articles/hello_universe.md").to_string(),
                    thumbnail: "/static/img/thumbnail3.png".to_string(),
                    header_img: "/static/img/thumbnail3.png".to_string(),
                    published: None,
                },
                ArticleModel {
                    id: 4,
                    title: "Au Revoir".to_string(),
                    author: "Lenz Koxholt".to_string(),
                    text: "".to_string(),
                    thumbnail: "/static/img/thumbnail4.jpg".to_string(),
                    header_img: "/static/img/thumbnail4.jpg".to_string(),
                    published: Some(date_from_str("2019-11-09")),
                },
                ArticleModel {
                    id: 5,
                    title: "Jo, Bis Dann!".to_string(),
                    author: "Lebinoir".to_string(),
                    text: "".to_string(),
                    thumbnail: "/static/img/thumbnail5.jpg".to_string(),
                    header_img: "/static/img/thumbnail5.jpg".to_string(),
                    published: Some(date_from_str("2019-04-01")),
                },
                ArticleModel {
                    id: 6,
                    title: "Hasta La Proxima!".to_string(),
                    author: "Glenz".to_string(),
                    text: "".to_string(),
                    thumbnail: "/static/img/thumbnail6.jpg".to_string(),
                    header_img: "/static/img/thumbnail6.jpg".to_string(),
                    published: Some(date_from_str("2019-02-28")),
                },
                ArticleModel {
                    id: 7,
                    title: "Ciao!".to_string(),
                    author: "Flens100".to_string(),
                    text: "".to_string(),
                    thumbnail: "/static/img/thumbnail7.png".to_string(),
                    header_img: "/static/img/thumbnail7.png".to_string(),
                    published: Some(date_from_str("2019-01-01")),
                },
                ArticleModel {
                    id: 8,
                    title: "Adios!".to_string(),
                    author: "xXWaXppXenXx".to_string(),
                    text: "".to_string(),
                    thumbnail: "/static/img/thumbnail8.png".to_string(),
                    header_img: "/static/img/thumbnail8.png".to_string(),
                    published: Some(date_from_str("2018-12-31")),
                },
            ],
        }
    }
}

fn date_from_str(s: &str) -> DateTime<Utc> {
    DateTime::from_utc(
        NaiveDateTime::parse_from_str(format!("{} 00:00:00", s).as_str(), "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        Utc,
    )
}
