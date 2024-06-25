use serde::{Deserialize, Serialize};
use crate::models::article::Article;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestArticleSchema<T> {
    pub article: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseArticleSchema {
    pub article: Article,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseArticlesSchema {
    pub articles: Vec<Article>,
    pub articles_count: usize,
}

// ---- List -----
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

// Read/Delete
#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateArticleSchema {
    pub title: String,
    pub description: String,
    pub body: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateArticleSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}
