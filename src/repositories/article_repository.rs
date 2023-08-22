use crate::model::ArticleModel;
use chrono::Utc;

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

impl Default for MockArticleRepository {
    fn default() -> Self {
        MockArticleRepository {
            articles: vec![
                ArticleModel {
                    id: 1,
                    title: "Hello World!".to_string(),
                    author: "Lenz".to_string(),
                    description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
                    text: include_str!("articles/hello_world.md").to_string(),
                    thumbnail: "/static/img/thumbnail.png".to_string(),
                    published: Some(Utc::now()),
                },
                ArticleModel {
                    id: 2,
                    title: "Hello Planet!".to_string(),
                    author: "Snens".to_string(),
                    description: "Ullamcorper morbi tincidunt ornare massa eget egestas.".to_string(),
                    text: include_str!("articles/hello_planet.md").to_string(),
                    thumbnail: "/static/img/thumbnail2.jpeg".to_string(),
                    published: Some(Utc::now()),
                },
                ArticleModel {
                    id: 3,
                    title: "Hello Universe!".to_string(),
                    author: "Wappen".to_string(),
                    description: "Accumsan lacus vel facilisis volutpat est velit egestas. Suspendisse interdum consectetur libero id faucibus.".to_string(),
                    text: include_str!("articles/hello_universe.md").to_string(),
                    thumbnail: "/static/img/thumbnail3.png".to_string(),
                    published: None,
                },
            ],
        }
    }
}

impl ArticleRepository for MockArticleRepository {
    fn find_all(&self) -> Vec<ArticleModel> {
        self.articles.clone()
    }

    fn find_by_id(&self, id: i32) -> Option<ArticleModel> {
        self.articles.iter().find(|a| a.id == id).cloned()
    }
}
