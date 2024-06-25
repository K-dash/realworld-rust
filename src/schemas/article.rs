use serde::{Deserialize, Serialize};
use crate::models::article::Article;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestSchema {
    pub article: CreateArticleSchema,
}

// ---- List -----
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSchema {
    pub articles: Vec<Article>,
    pub articles_count: usize,
}

// Read/Delete
#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub slug: String,
}

// Create
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateArticleSchema {
    pub title: String,
    pub description: String,
    pub body: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub tags: Option<Vec<String>>,
}

// Update
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateArticleSchema {
    pub slug: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_published: Option<bool>,
}
