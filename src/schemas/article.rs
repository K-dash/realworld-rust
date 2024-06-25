use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ArticleRequestData {
    pub article: CreateArticleSchema,
}

// List
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
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
